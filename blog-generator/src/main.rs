use std::env::args;
use std::fs;
use std::path::PathBuf;

use anyhow::Error;
use tera::Tera;

mod discover;
mod index;
mod parse;
mod postprocessing;
mod render;
mod types;
mod utils;
mod write;
use discover::discover;
use index::index;
use parse::parse;
use render::render;
use write::write;
use types::Config;

fn main() -> Result<(), Error> {
    let config_path = args().nth(1).map(PathBuf::from)
        .expect("usage: ssg <config.toml>");

    let config_str = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&config_str)?;

    let res = parse(&discover(&config.input_dir))?;
    let tera = Tera::new(
        config.templates_dir.join("*")
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("templates_dir path could not be read"))?
    )?;

    let site_index = index(res);
    let pages_to_write = render(&site_index, &tera, &config.input_dir)?;
    write(
        pages_to_write,
        &config.output_dir
    )?;
    
    Ok(())
}
