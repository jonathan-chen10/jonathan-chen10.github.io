use std::env::args;
use std::fs;
use std::path::PathBuf;

use anyhow::Error;

mod discover;
mod index;
mod parse;
mod postprocessing;
mod types;
mod utils;
mod write;
use discover::discover;
use index::index;
use parse::parse;
use write::write;
use types::Config;

fn main() -> Result<(), Error> {
    let config_path = args().nth(1).map(PathBuf::from)
        .expect("usage: ssg <config.toml>");

    let config_str = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&config_str)?;

    match parse(&discover(&config.input_dir)) {
        Ok(res) => {
            let idx = index(res);
            for p in &idx.posts {
                println!("{}", p.meta.title);
            }
            println!();
            for (tag, posts) in &idx.tags {
                println!("{} :", tag);
                for p in posts {
                    println!("- {}", p.meta.title);
                }
            }

            use types::OutputFile;
            let test_ofile = OutputFile {
                html: "<!doctype html><html><body>testing</body></html>".to_string(),
                path_relative: "test-write/output.html".into(),
                path_assets: Some(config.input_dir.join("test")),
            };
            write(vec![test_ofile], &config.output_dir)?;
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    Ok(())
}
