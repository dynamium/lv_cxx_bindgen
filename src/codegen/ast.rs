use crate::{conf::CxxVersion, parse::TypedValue};
use core::fmt::Debug;

pub trait Node: Debug {
    fn gen_source(&self, target: &CxxVersion) -> String;
}

#[derive(Debug)]
pub struct FunctionDeclaration<'a> {
    pub return_type: &'a str,
    pub identifier: &'a str,
    pub args: &'a [TypedValue],
    pub body: &'a [Box<dyn Node>],
}

#[derive(Debug)]
pub struct VariableDeclaration<'a, N: Node> {
    pub kind: &'a str,
    pub identifier: &'a str,
    pub assignment: N,
}

#[derive(Debug)]
pub struct FunctionCall<'a> {
    pub path: &'a [&'a str],
    pub identifier: &'a str,
    pub args: &'a [&'a str],
}

#[derive(Debug)]
pub struct NamespaceDeclaration<'a> {
    pub identifier: &'a str,
    pub members: Vec<FunctionDeclaration<'a>>,
}
