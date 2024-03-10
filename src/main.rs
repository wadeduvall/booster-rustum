use booster_rustum::parser;
use booster_rustum::Args;

use clap::Parser;
use color_eyre::eyre::Result;
use log::error;

use std::fs;

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();
    Args::parse();

    let file = match fs::File::open("config/boster-rustum.yaml") {
        Ok(file) => file,
        Err(e) => {
            error!("Couldn't open booster-rustum.yaml: {e}");
            return Ok(());
        }
    };
    let yaml: parser::Config = match serde_yaml::from_reader(file) {
        Ok(yaml) => yaml,
        Err(e) => {
            error!("Cannot serialize: {e}");
            return Ok(());
        }
    };
    println!("Deserialize =\n{yaml:?}");

    Ok(())
}
