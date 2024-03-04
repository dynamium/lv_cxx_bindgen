use crate::{api_map::APIMap, conf, cli};
use log::debug;

mod class;
mod enumeration;
mod func;
mod namespace;

use enumeration::enumeration_processor;
use func::function_processor;
use namespace::namespace_generator;

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

#[derive(Debug, Clone)]
pub struct EnumerationMember {
    pub identifier: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Enumeration {
    pub identifier: Option<String>,
    pub members: Vec<EnumerationMember>,
}

pub fn make_hl_ast(api_map: APIMap, conf: &conf::Config, anon_enum_handling : &cli::AnonEnumGeneration) {
    debug!("Generation config: {:#?}", conf);
    let functions = function_processor(&api_map, &conf.functions);
    debug!("Functions: {functions:#?}");
    let namespaces = namespace_generator(&functions, &conf.namespaces);
    debug!("Namespaces: {namespaces:#?}");
    let enumerations = enumeration_processor(&api_map, anon_enum_handling);
    debug!("Enumerations: {enumerations:#?}");
}
