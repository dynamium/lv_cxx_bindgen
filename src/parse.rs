use std::{fs, path::PathBuf};

use anyhow::{bail, Result};
use log::debug;
use tree_sitter::{Node, Parser};

#[derive(Debug)]
pub struct Function {
    pub return_type: String,
    pub identifier: String,
    pub args: Vec<TypedValue>,
    pub original_ident: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TypedValue {
    pub identifier: Option<String>,
    // type
    pub kind: String,
}

pub fn get_header_functions(input: &[PathBuf]) -> Result<Vec<Function>> {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_cpp::language())?;
    let mut functions = vec![];

    for path in input {
        debug!("Parsing file {:?}", path);
        let file_str = fs::read_to_string(path)?;
        let tree = parser.parse(&file_str, None).unwrap();
        let root = tree.root_node();

        let function = scan_for_functions(root, &file_str);

        functions.push(function);
    }

    Ok(functions.into_iter().flatten().collect())
}

fn scan_for_functions(node: Node, source_str: &str) -> Vec<Function> {
    let mut functions = vec![];
    for i in 0..node.named_child_count() {
        let child = node.named_child(i).unwrap();
        debug!("{:?}", child.kind());

        if child.kind() == "preproc_ifdef"
            || child.kind() == "linkage_specification"
            || child.kind() == "declaration_list"
        {
            debug!("Walking into new node");
            functions.append(&mut scan_for_functions(child, source_str));
        }

        if child.kind() == "declaration" {
            debug!("Found declaration");
            let function = parse_function_declaration(child, source_str);
            if let Some(function) = function {
                debug!("Parsed function: {:#?}", function);
                functions.push(function);
            }
        }
    }
    functions
}

fn parse_function_declaration(node: Node, source_str: &str) -> Option<Function> {
    let type_range = node.named_child(0)?.range();
    let mut type_str = source_str[type_range.start_byte..type_range.end_byte].to_string();
    debug!("Type: {:?}", type_str);

    let declarator_node;
    if node.child_by_field_name("declarator").unwrap().kind() == "pointer_declarator" {
        debug!("Function type is a pointer");
        type_str.push('*');
        declarator_node = node
            .child_by_field_name("declarator")
            .unwrap()
            .named_child(0)
            .unwrap();
    } else {
        declarator_node = node.child_by_field_name("declarator").unwrap();
    }
    if declarator_node.kind() != "function_declarator" {
        debug!(
            "Encountered declaration is not a function, but rather a {}, skipping",
            declarator_node.kind()
        );
        return None;
    }
    let function_name_node = declarator_node.child_by_field_name("declarator").unwrap();
    let function_name_str =
        &source_str[function_name_node.range().start_byte..function_name_node.range().end_byte];
    debug!("Function name: {:?}", function_name_str);

    let parameters_node = declarator_node.child_by_field_name("parameters").unwrap();

    let mut parameters = vec![];
    debug!(
        "Parameters node: {}, {}, {}",
        parameters_node.to_sexp(),
        &source_str[parameters_node.range().start_byte..parameters_node.range().end_byte],
        parameters_node.child_count()
    );

    for i in 0..parameters_node.named_child_count() {
        debug!("At parameter node {}", i);
        let parameter_declaration = parameters_node.named_child(i).unwrap();
        debug!(
            "Parameter declaration: {} {}",
            parameter_declaration.to_sexp(),
            &source_str
                [parameter_declaration.range().start_byte..parameter_declaration.range().end_byte]
        );
        let parameter_type = parameter_declaration.child(0).unwrap();
        let mut type_str = source_str
            [parameter_type.range().start_byte..parameter_type.range().end_byte]
            .to_string();

        let mut identifier_str = None;
        if let Some(identifier) = parameter_declaration.named_child(1) {
            let parameter_identifier;
            if identifier.kind() == "pointer_declarator" {
                debug!("Paremeter type is a pointer");
                parameter_identifier = parameter_declaration
                    .named_child(1)
                    .unwrap()
                    .named_child(0)
                    .unwrap();
                type_str.push('*');
            } else {
                parameter_identifier = parameter_declaration.named_child(1).unwrap();
            }
            identifier_str = Some(
                source_str[parameter_identifier.range().start_byte
                    ..parameter_identifier.range().end_byte]
                    .to_string(),
            );
            debug!("Identifier exists: {:?}", identifier_str);
        }

        let typed_value = TypedValue {
            identifier: identifier_str,
            kind: type_str.to_string(),
        };
        debug!("Resulting parameter: {:?}", typed_value);

        parameters.push(typed_value);
    }

    debug!("Parameters: {:?}", parameters);
    parameters = parameters
        .into_iter()
        .filter(|param| param.identifier != None && param.kind != "void")
        .collect();

    Some(Function {
        return_type: type_str.to_string(),
        identifier: function_name_str.to_string(),
        args: parameters,
        original_ident: None,
    })
}

pub fn get_header_paths(input: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_cpp::language())?;
    let mut headers = vec![];

    for path in input {
        let file_str = fs::read_to_string(path)?;
        let tree = parser.parse(&file_str, None).unwrap();
        let root = tree.root_node();
        headers.append(&mut scan_for_header_paths(root, &file_str)?.to_vec());
    }

    Ok(headers)
}

fn scan_for_header_paths(node: Node, source_str: &str) -> Result<Vec<PathBuf>> {
    let mut paths = vec![];

    for i in 0..node.named_child_count() {
        let child = node.named_child(i).unwrap();
        debug!("{:?}", child.kind());

        if child.kind() == "preproc_ifdef"
            || child.kind() == "linkage_specification"
            || child.kind() == "declaration_list"
        {
            debug!("Walking into new node");
            paths.append(&mut scan_for_header_paths(child, source_str)?.to_vec());
        }

        if child.kind() == "preproc_include" {
            debug!("Found preproc_include");
            let path_node = child.named_child(0).unwrap();
            debug!("String literal: {}", path_node.to_sexp());
            match path_node.kind() {
                "string_literal" => {
                    let string_content_node = path_node.named_child(0).unwrap();
                    let node_str = &source_str[string_content_node.range().start_byte..string_content_node.range().end_byte];
                    debug!("Got string_literal content: {}", node_str);
                    paths.push(node_str.into());
                },
                "system_lib_string" => {
                    let node_str = &source_str[path_node.range().start_byte..path_node.range().end_byte];
                    debug!("Got system_lib_string content: {}", node_str);
                    paths.push(node_str.into());
                },
                _ => bail!("Unknown string type for an #include statement encountered. Please report this to developers.")
            }
        }
    }

    Ok(paths)
}
