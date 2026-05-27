use std::env::args;
use std::fs;
use std::path::PathBuf;

use anyhow::Error;
use serde::Deserialize;

mod discover;
mod parse;
mod postprocessing;
mod types;
mod utils;
use discover::discover;
use parse::parse;

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

    match parse(&discover(&config.input_dir)) {
        Ok(res) => {
            for post in res {
                println!("{}", post.meta.title);
                println!("{}", post.meta.date_created);
                println!("{}", post.slug);
                println!("{}", post.html);
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    Ok(())
}
