use crate::{api_map::APIMap, conf};
use log::debug;

mod class;
mod func;
mod namespace;
mod enumeration;

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

pub fn make_hl_ast(api_map: APIMap, conf: &conf::Generation) {
    debug!("Generation config: {:#?}", conf);
    let functions = func::function_processor(&api_map, &conf.functions);
    debug!("Functions: {functions:#?}");
    let namespaces = namespace::namespace_generator(&functions, &conf.namespaces);
    debug!("Namespaces: {namespaces:#?}");
    let enumerations = enumeration::enumeration_processor(&api_map);
    debug!("Enumerations: {enumerations:#?}");
}
