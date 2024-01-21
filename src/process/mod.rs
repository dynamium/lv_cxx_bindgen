use crate::api_map::{APIMap, FuncArg};
use log::debug;
use std::collections::HashSet;

use self::func::function_processor;

mod class;
mod func;
mod namespace;

#[derive(Debug, Clone)]
pub struct Namespace {
    pub identifier: String,
    pub members: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub identifier: String,
    pub constructor_args: Vec<Argument>,
    pub members: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub identifier: String,
    pub return_type: String,
    pub args: Vec<Argument>,
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub identifier: Option<String>,
    pub kind: String,
}

pub fn make_hl_ast(api_map: APIMap) {
    let functions = function_processor(&api_map);
}
