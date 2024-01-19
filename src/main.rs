mod api_map;
mod cli;
mod codegen;
mod conf;
mod process;
mod template;

use anyhow::{Context, Result};
use clap::Parser;
use log::{debug, info};
use simplelog::{ColorChoice, TermLogger, TerminalMode};
use std::{fs, path::PathBuf};

use crate::{cli::Cli, conf::Config, process::make_api_map};

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

    info!("Retrieving all functions...");
    let api_map_file_path = config.input.cwd.unwrap_or(PathBuf::new()).join("lvgl.json");
    let api_map_file_content = fs::read_to_string(api_map_file_path)?;
    let api_map = api_map::parse(&api_map_file_content)?;

    debug!("Parsed API map: {:#?}", api_map);

    info!("Generating C++ API...");
    let cpp_api_map = make_api_map(api_map);
    debug!("C++ API: {cpp_api_map:#?}");
    // let namespaces_list =
    //     group::group_in_namespaces(&config.generation.namespace_exclude, &functions_orig);
    // debug!("Resulting namespaces: {:#?}", namespaces_list);

    // info!("Grouping in classes...");
    // let _class_list = group::group_in_classes(&config.generation.class_exclude, &functions_orig);

    // info!("Converting groups into AST...");

    // let mut namespaces_ast: Vec<Box<dyn Node>> = vec![];
    // for namespace in &namespaces_list.0 {
    //     let mut members: Vec<Box<dyn Node>> = vec![];
    //     for member in &namespace.members {
    //         let member = member.clone();
    //         members.push(Box::new(FunctionDeclaration {
    //             return_type: member.return_type,
    //             identifier: member.identifier,
    //             args: member.args,
    //             body: vec![],
    //         }));
    //     }

    //     namespaces_ast.push(Box::new(NamespaceDeclaration {
    //         identifier: namespace.identifier.clone(),
    //         members,
    //     }));
    // }

    // debug!("Resulting AST: {:#?}", namespaces_ast);
    // let namespace_ast = NamespaceDeclaration {
    //     identifier: "lvgl".to_string(),
    //     members: namespaces_ast,
    // };
    // debug!(
    //     "Codegen: {}",
    //     namespace_ast.gen_source(&config.generation.target)
    // );
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
