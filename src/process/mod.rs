use crate::{api_map::APIMap, conf};
use log::debug;

mod class;
mod enumeration;
mod func;
mod namespace;
mod structure;

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

#[derive(Debug, Clone)]
pub struct StructureField {
    pub identifier: String,
    pub kind: String,
    pub bitsize: Option<u8>,
}

#[derive(Debug, Clone)]
pub struct Structure {
    pub identifier: String,
    pub fields: Vec<StructureField>,
}

pub fn make_hl_ast(api_map: APIMap, conf: &conf::Generation) {
    debug!("Generation config: {:#?}", conf);
    let functions = func::function_processor(&api_map, &conf.functions);
    debug!("Functions: {functions:#?}");
    let namespaces = namespace::namespace_generator(&functions, &conf.namespaces);
    debug!("Namespaces: {namespaces:#?}");
    let enumerations = enumeration::enumeration_processor(&api_map);
    debug!("Enumerations: {enumerations:#?}");
    let structures = structure::structure_processor(&api_map);
    debug!("Structures: {structures:#?}");
}
