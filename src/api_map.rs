use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct JSONRoot {
    pub enums: Vec<JSONValue>,
    pub functions: Vec<JSONValue>,
    pub structures: Vec<JSONValue>,
    pub unions: Vec<JSONValue>,
    pub variables: Vec<JSONValue>,
    pub typedefs: Vec<JSONValue>,
    pub forward_decls: Vec<JSONValue>,
    pub macros: Vec<JSONValue>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JSONValue {
    pub name: Option<String>,
    pub json_type: JSONType,
    pub docstring: Option<String>,
    pub quals: Option<Vec<String>>,
    pub storage: Option<Vec<String>>,
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
    PoifunctionPointerter, // FIXME: remove this after fix of the parser
    Variable,
    Union,
    #[serde(rename = "forward_decl")]
    ForwardDeclaration,
    Macro,
}

pub struct Enum<T> {
    pub identifier: String,
    pub members: Vec<(String, T)>,
}

pub struct Function {
    pub identifier: String,
    pub return_type: String,
    pub args: Vec<FuncArg>,
}

pub struct FuncArg {
    pub identifier: String,
    pub r#type: String,
}

pub struct Struct {
    pub identifier: String,
    pub fields: Vec<StructField>,
}

pub struct StructField {
    pub identifier: String,
    pub r#type: String,
    pub bitsize: Option<u8>,
}

pub fn parse(source_str: &str) -> Result<JSONRoot> {
    return Ok(serde_json::from_str(source_str)?);
}
