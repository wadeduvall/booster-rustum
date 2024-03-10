use booster_rustum::parser;
use booster_rustum::Args;
use clap::Parser;
use color_eyre::eyre::Result;
use std::fs;

fn main() -> Result<()> {
    color_eyre::install()?;
    Args::parse();

    let file =
        fs::File::open("config/booster-rustum.yaml").expect("Couldn't open booster-rustum.yaml");
    let yaml: parser::Config = serde_yaml::from_reader(file).expect("Cannot serialize");
    println!("Deserialize =\n{yaml:?}");

    Ok(())
}
