mod cli;
mod codegen;
mod conf;
mod group;
mod parse;

use anyhow::Result;
use clap::Parser;
use log::{debug, info, warn, LevelFilter};
use simplelog::{ColorChoice, TermLogger, TerminalMode};
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
            ColorChoice::Auto,
        );
    } else {
        _ = TermLogger::init(
            LevelFilter::Trace,
            simplelog::Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        );
    }
    info!("Starting generation...");
    info!("Input files: {:?}", &config.input.files.clone());
    let mut input_files;
    if config.input.auto_scan.is_some_and(|x| x) {
        info!("Running auto scan...");
        let mut binding = config.input.files.clone();
        if let Some(cwd) = &config.input.cwd {
            binding = binding
                .into_iter()
                .map(|path| {
                    let mut new_path = cwd.clone();
                    new_path.push(path);
                    new_path
                })
                .collect();
            debug!("Input files with cwd applied: {:#?}", binding);
        }
        input_files = parse::get_header_paths(&binding)?;
        debug!("Parsed paths: {:#?}", input_files);
    } else {
        input_files = config.input.files;
    }
    if let Some(cwd) = config.input.cwd {
        input_files = input_files
            .into_iter()
            .map(|path| {
                let mut new_path = cwd.clone();
                new_path.push(path);
                new_path
            })
            .collect();
        debug!("Input files with cwd applied: {:#?}", input_files);
    }

    info!("Retrieving all functions...");
    let functions_orig = parse::get_header_functions(&input_files)?;

    debug!("Parsed functions: {:#?}", functions_orig);

    info!("Grouping in namespaces...");
    let namespaces_list =
        group::group_in_namespaces(&config.generation.namespaces, &functions_orig);
    debug!("Resulting namespaces: {:#?}", namespaces_list);

    warn!("Grouping in classes is not implemented yet!");
    // info!("Grouping in classes...");
    // let class_list = group::group_in_classes(&config.generation.classes, &functions_orig);

    Ok(())
}
