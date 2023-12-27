mod cli;
mod conf;
mod parse;

use anyhow::Result;
use clap::Parser;
use log::info;
use std::fs;

use crate::{cli::Cli, conf::Config};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config: Config = toml::from_str(fs::read_to_string("conf.toml")?.as_str())?;

    colog::init();
    if !cli.verbose {
        log::set_max_level(log::LevelFilter::Warn);
    }
    info!("Starting generation...");

    let headers = parse::headers(&config.input_files.clone());

    Ok(())
}
