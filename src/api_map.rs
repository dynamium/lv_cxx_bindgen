use serde::Deserialize;
use anyhow::Result;

#[derive(Deserialize, Debug, Clone)]
pub struct JSONRoot {
    pub enums: Vec<JSONValue>,
    pub functions: Vec<JSONValue>,
    pub structures: Vec<JSONValue>,
    pub unions: Vec<JSONValue>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JSONValue {
    pub name: Option<String>,
    pub json_type: JSONType,
    pub r#type: Option<Box<JSONValue>>, // wrapped in a Box to fix type recursion
    pub fields: Option<Vec<JSONValue>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum JSONType {
    PrimitiveType,
    StdlibType,
    LvglType,
    EnumMember,
    Field,
    Struct,
    Typedef,
    Enum,
    Function,
    Pointer,
    Array,
    #[serde(rename = "ret_type")]
    ReturnType,
    PoifunctionPointerter, // remove this after fix of the parser
    Variable,
    Union,
    #[serde(rename = "forward_decl")]
    ForwardDeclaration
}

pub struct Enum<T> {
    pub identifier: String,
    pub members: Vec<(String, T)>
}

pub struct Function {
    pub identifier: String,
    pub return_type: String,
    pub args: Vec<FuncArg>
}

pub struct FuncArg {
    pub identifier: String,
    pub r#type: String
}

pub struct Struct {
    pub identifier: String,
    pub fields: Vec<StructField>
}

pub struct StructField {
    pub identifier: String,
    pub r#type: String,
    pub bitsize: Option<u8>
}

pub struct Typedef {
    pub identifier: String,
}

pub fn parse(source_str: &str) -> Result<JSONRoot> {
    return Ok(serde_json::from_str(source_str)?);
}
