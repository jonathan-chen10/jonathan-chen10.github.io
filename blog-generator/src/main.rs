use std::env::args;
use std::fs;
use std::path::PathBuf;

use anyhow::Error;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    input_dir: PathBuf,
    output_dir: PathBuf,
    templates_dir: PathBuf,
}

fn main() -> Result<(), Error> {
    let config_path = args().nth(1).map(PathBuf::from)
        .expect("usage: ssg <config.toml>");

    let config_str = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&config_str)?;

    println!("{:#?}", config);
    Ok(())
}
