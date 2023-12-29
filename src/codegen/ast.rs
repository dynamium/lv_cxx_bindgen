use std::fmt::{Display, write};

use log::warn;

use crate::parse::TypedValue;

pub trait Node: Display {}

pub struct FunctionDeclaration<'a> {
    pub return_type: &'a str,
    pub identifier: &'a str,
    pub args: &'a[TypedValue],
    pub body: &'a[Box<dyn Node>]
}

pub struct VariableDeclaration<'a, N: Node> {
    pub kind: &'a str,
    pub identifier: &'a str,
    pub assignment: N 
}

pub struct FunctionCall<'a> {
    pub path: &'a[&'a str],
    pub identifier: &'a str,
    pub args: &'a[&'a str]
}
