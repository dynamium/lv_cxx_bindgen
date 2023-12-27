use std::{fs};

use anyhow::Result;
use log::info;
use tree_sitter::{Parser, Tree, Node};

pub struct Function {
    pub name: String,
}

pub fn headers(input: &[String]) -> Result<Vec<Tree>> {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_cpp::language())?;
    let mut trees = vec![];

    for path in input {
        let file_str = fs::read_to_string(path)?;
        let tree = parser.parse(file_str, None).unwrap();
        let root = tree.root_node();
        info!("{:?}", root.to_sexp());

        walk_node(root);

        trees.push(tree);
    }

    return Ok(trees);
}

fn walk_node(node: Node) {
    for i in 0..node.named_child_count() {
        let child = node.named_child(i).unwrap();
        info!("{:?}", child.kind());

        if child.kind() == "preproc_ifdef"
            || child.kind() == "linkage_specification"
            || child.kind() == "declaration_list" {
            info!("New node");
            walk_node(child);
        }
    }
}
