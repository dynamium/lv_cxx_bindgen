use crate::{conf::CxxVersion, parse::TypedValue};
use core::fmt::Debug;

pub trait Node: Debug {
    fn gen_source(&self, target: &CxxVersion) -> String;
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub return_type: String,
    pub identifier: String,
    pub args: Vec<TypedValue>,
    pub body: Vec<Box<dyn Node>>,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub path: Vec<String>,
    pub identifier: String,
    pub args: Vec<String>,
}

#[derive(Debug)]
pub struct NamespaceDeclaration<'a> {
    pub identifier: &'a str,
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
