# Clap and Serde derive

[![Crates.io](https://img.shields.io/crates/v/clap-serde-derive?style=flat-square&logo=rust)](https://crates.io/crates/clap-serde-derive)
[![Crates.io](https://img.shields.io/crates/d/clap-serde-derive?style=flat-square&logo=rust)](https://crates.io/crates/clap-serde-derive)
[![Docs.rs](https://img.shields.io/badge/docs.rs-clap--serde--derive-1234?style=flat-square&logo=docs.rs)](https://docs.rs/clap-serde-derive)
[![License](https://img.shields.io/gitlab/license/DPDmancul/clap-serde-derive?color=brightgreen&style=flat-square&logo=gnu)](https://gitlab.com/DPDmancul/clap-serde-derive/-/blob/main/LICENSE.md)
[![GitLab pipeline](https://img.shields.io/gitlab/pipeline/DPDmancul/clap-serde-derive/main?label=tests&style=flat-square&logo=gitlab)](https://gitlab.com/DPDmancul/clap-serde-derive/builds)

<div align="center">
    <img alt="" src="https://gitlab.com/DPDmancul/clap-serde-derive/-/raw/main/assets/logo.svg" />
</div>

With the `ClapSerde` procedural macro both clap and serde can be derived from a struct.  
Then the struct can be parsed from clap and serde sources as in a layered config: the last
source has the precedence.

```rust ignore
Args::from(serde_parsed)
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
use clap_serde_derive::{
    clap::{self, ArgAction},
    serde::Serialize,
    ClapSerde,
};

#[derive(ClapSerde, Serialize)]
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
    #[clap(long = "num")]
    pub clap_num: u32,

    /// Skip clap
    #[serde(rename = "number")]
    #[clap(skip)]
    pub serde_num: u32,

    /// Recursive fields
    #[clap_serde]
    #[clap(flatten)]
    pub suboptions: SubConfig,
}

#[derive(ClapSerde, Serialize)]
#[derive(Debug)]
pub struct SubConfig {
    #[default(true)]
    #[clap(long = "no-flag", action = ArgAction::SetFalse)]
    pub flag: bool,
}

let args = Args::from(serde_yaml::from_str::<<Args as ClapSerde>::Opt>("number: 12").unwrap())
    .merge_clap();
assert_eq!(
    serde_yaml::to_string(&args).unwrap(),
    serde_yaml::to_string(&Args {
        serde_num: 12,
        clap_num: 13,
        ..Args::default()
    })
    .unwrap(),
);
```

## Config path from command line

You can easily take the config file path from command line in this way.

```rust
use std::{fs::File, io::BufReader};

use clap_serde_derive::{
    clap::{self, Parser},
    ClapSerde,
};

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
    pub config: <Config as ClapSerde>::Opt,
}

#[derive(ClapSerde)]
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
    match serde_yaml::from_reader::<_, <Config as ClapSerde>::Opt>(BufReader::new(f)) {
        // merge config already parsed from clap
        Ok(config) => Config::from(config).merge(&mut args.config),
        Err(err) => panic!("Error in configuration file:\n{}", err),
    }
} else {
    // If there is not config file return only config parsed from clap
    Config::from(&mut args.config)
};
```
