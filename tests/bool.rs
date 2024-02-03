use clap_serde_derive::{
    clap::{self, Parser, error::Error},
    ClapSerde,
};

#[derive(Parser)]
struct Args {
    #[command(flatten)]
    pub config: <Config as ClapSerde>::Opt,
}

#[derive(ClapSerde)]
struct Config {
    #[arg(short, long)]
    test_flag: bool,
}

#[test]
fn bool_present() -> Result<(), Error> {
    let res = Args::try_parse_from(["cmd", "-t"]);
    assert_eq!(res?.config.test_flag, Some(true));
    Ok(())
}

#[test]
fn bool_absent() -> Result<(), Error> {
    let res = Args::try_parse_from(["cmd"]);
    assert_eq!(res?.config.test_flag, None);
    Ok(())
}
