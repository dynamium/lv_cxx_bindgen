mod cli;
mod codegen;
mod conf;
mod group;
mod parse;

use anyhow::{Context, Result};
use clap::Parser;
use log::{debug, info};
use simplelog::{ColorChoice, TermLogger, TerminalMode};
use std::fs;

use crate::{
    cli::Cli,
    codegen::ast::{FunctionDeclaration, NamespaceDeclaration, Node},
    conf::Config,
};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut config: Config = toml::from_str(
        fs::read_to_string(cli.config)
            .context("Failed to read the config file")?
            .as_str(),
    )?;
    if let Some(target) = cli.target {
        config.generation.target = target;
    }

    _ = TermLogger::init(
        cli.verbose.log_level_filter(),
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );
    info!("Starting generation...");
    debug!("Input files: {:?}", &config.input.files.clone());
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

    info!("Grouping in classes...");
    let _class_list = group::group_in_classes(&config.generation.classes, &functions_orig);

    info!("Converting groups into AST...");

    let mut namespaces_ast: Vec<Box<dyn Node>> = vec![];
    for namespace in &namespaces_list.0 {
        let mut members: Vec<Box<dyn Node>> = vec![];
        for member in &namespace.members {
            let member = member.clone();
            members.push(Box::new(FunctionDeclaration {
                return_type: member.return_type,
                identifier: member.identifier,
                args: member.args,
                body: vec![],
            }));
        }

        namespaces_ast.push(Box::new(NamespaceDeclaration {
            identifier: namespace.identifier.clone(),
            members,
        }));
    }

    debug!("Resulting AST: {:#?}", namespaces_ast);
    let namespace_ast = NamespaceDeclaration {
        identifier: "lvgl".to_string(),
        members: namespaces_ast
    };
    debug!("Codegen: {}", namespace_ast.gen_source(&config.generation.target));
    // let mut ast = vec![];
    // for namespace in &namespaces_ast {
    //     ast.push(namespace.gen_source(&config.generation.target));
    // }
    // debug!(
    //     "Codegen for first namespace: {}",
    //     namespaces_ast[0].gen_source(&conf::CxxVersion::Cxx20)
    // );
    // debug!("Generated source code: {}", namespaces_ast[0]);

    Ok(())
}
