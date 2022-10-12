# Clap and Serde derive

With the [`clap_serde`] procedural macro both clap and serde can be derived from a struct.  
Then the struct can be parsed from clap and serde sources as in a layered config: the last
source has the precedence.

```rust ignore
Args::from_serde(...)
    .unwrap()
    .merge_clap();
```

In the snippet the precedence is:

1. Command line from clap;
2. Config file from serde;
3. Default values.

## Example

In this example we define a struct which derives both clap and serde.
The struct has various parameter type and also various attributes on its fields.

Finally we parse the structure from a YAML file with serde and then from command line with
clap. The arguments from clap will override those from serde; the default value will be used if
no source contained the field.

```rust
use clap_serde_derive::{clap_serde, ClapSerde, clap::ArgAction};

#[clap_serde]
#[derive(Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// Input files
    pub input: Vec<std::path::PathBuf>,

    /// String argument
    #[clap(short, long)]
    name: String,

    /// Skip serde deserialize
    #[default(13)]
    #[serde(skip_deserializing)]
    #[clap(long="num")]
    pub clap_num: u32,

    /// Skip clap
    #[serde(rename="number")]
    #[clap(skip)]
    pub serde_num: u32,

    /// Recursive fields
    #[clap_serde]
    #[clap(flatten)]
    pub suboptions: SubConfig,
}

#[clap_serde]
#[derive(Debug)]
pub struct SubConfig {
    #[default(true)]
    #[clap(long = "no-flag", action = ArgAction::SetFalse)]
    pub flag: bool
}

let args = Args::from_serde(serde_yaml::from_str("number: 12"))
    .unwrap()
    .merge_clap();
assert_eq!(
    serde_yaml::to_string(&args).unwrap(),
    serde_yaml::to_string(&Args {
        serde_num: 12,
        clap_num: 13,
        ..Args::default()
    }).unwrap(),
);
```

## Config path from command line

You can easily take the config file path from command line in this way.

```rust
use std::{fs::File, io::BufReader};

use clap_serde_derive::{clap_serde, ClapSerde, clap::{Parser, ArgAction}};

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// Input files
    input: Vec<std::path::PathBuf>,

    /// Config file
    #[clap(short, long = "config", default_value = "config.yml")]
    config_path: std::path::PathBuf,

    /// Rest of arguments
    #[clap(flatten)]
    pub config: <Config as ClapSerde<'static>>::Opts,
}

#[clap_serde]
struct Config {
    /// String argument
    #[clap(short, long)]
    name: String,
}

// Parse whole args with clap
let mut args = Args::parse();

// Get config file
let config = if let Ok(f) = File::open(&args.config_path) {
    // Parse config with serde
    match Config::from_serde(serde_yaml::from_reader(BufReader::new(f))) {
        // merge config already parsed from clap
        Ok(config) => config.merge(&mut args.config),
        Err(err) => panic!("Error in configuration file:\n{}", err)
    }
} else {
    // If there is not config file return only config parsed from clap
    Config::from_opt(&mut args.config)
};
```
