use core::fmt::Debug;

use crate::cli::CxxVersion;

pub trait Node: Debug {
    fn gen_source(&self, target: &CxxVersion) -> String;
}

#[derive(Debug)]
pub struct TypedIdentifier {
    pub identifier: String,
    pub kind: String,
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub return_type: String,
    pub identifier: String,
    pub args: Vec<TypedIdentifier>,
    pub body: Vec<Box<dyn Node>>,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub path: Vec<String>,
    pub identifier: String,
    pub args: Vec<String>,
}

#[derive(Debug)]
pub struct NamespaceDeclaration {
    pub identifier: String,
    pub members: Vec<Box<dyn Node>>,
}

#[derive(Debug)]
pub struct VariableDeclaration<'a, N: Node> {
    pub is_static: bool,
    pub kind: &'a str,
    pub identifier: &'a str,
    pub assignment: Option<N>,
}

#[derive(Debug)]
pub struct Comment<'a> {
    pub content: &'a str,
    pub multiline: bool,
}
