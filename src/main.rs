mod cli;
mod conf;
mod parse;

use anyhow::Result;
use clap::Parser;
use log::{info, trace, LevelFilter};
use simplelog::{TermLogger, TerminalMode, ColorChoice};
use std::fs;

use crate::{cli::Cli, conf::Config};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config: Config = toml::from_str(fs::read_to_string("lv_cxx_bindgen.toml")?.as_str())?;

    if !cli.verbose {
        _ = TermLogger::init(
            LevelFilter::Info,
            simplelog::Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto
            );
    } else {
        _ = TermLogger::init(
            LevelFilter::Trace,
            simplelog::Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto
            );
    }
    info!("Starting generation...");
    info!("Input files: {:?}", &config.input.files.clone());

    let headers = parse::get_header_functions(&config.input.files.clone());
    let binding = config.input.files.clone();
    let paths = parse::get_header_paths(&binding);

    info!("Parsed functions: {:#?}", headers);
    info!("Parsed paths: {:#?}", paths);

    Ok(())
}
