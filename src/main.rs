use booster_rustum::parser;
use booster_rustum::Args;

use clap::Parser;
use log::error;

use std::fs;
use std::process::ExitCode;

fn main() -> ExitCode {
    if color_eyre::install().is_err() {
        error!("Failed to init colo_eyre.");
        return ExitCode::from(2);
    };
    env_logger::init();
    Args::parse();

    let file = match fs::File::open("config/booster-rustum.yaml") {
        Ok(file) => file,
        Err(e) => {
            error!("Couldn't open booster-rustum.yaml: {e}");
            return ExitCode::from(2);
        }
    };
    let yaml: parser::Config = match serde_yaml::from_reader(file) {
        Ok(yaml) => yaml,
        Err(e) => {
            error!("Cannot serialize: {e}");
            return ExitCode::from(2);
        }
    };
    println!("Deserialize =\n{yaml:?}");
    ExitCode::from(0)
}
