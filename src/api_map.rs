use serde::Deserialize;

#[derive(Deserialize)]
pub struct Root {
    pub enums: Vec<Enum>,
    pub functions: Vec<Function>,
    pub structures: Vec<Struct>,
    pub unions: Vec<Union>
}

#[derive(Deserialize)]
pub struct Type {
    pub name: Option<String>,
    pub json_type: JSONType,
    pub r#type: Option<Box<Type>>, // wrapped in a Box to fix type recursion
    pub fields: Vec<StructField>
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JSONType {
    PrimitiveType,
    StdlibType,
    LvType,
    EnumMember,
    Field,
    Struct,
    Typedef
}

#[derive(Deserialize)]
pub struct Enum {
    pub name: String,
    pub r#type: Type,
    pub json_type: JSONType,
    pub members: String
}

#[derive(Deserialize)]
pub struct Function {
    pub name: String,
    pub args: Vec<FunctionArg>,
    pub json_type: JSONType,
    pub r#type: Type,
}

#[derive(Deserialize)]
pub struct FunctionArg {
    pub name: String,
    pub json_type: JSONType,
    pub r#type: Type
}

#[derive(Deserialize)]
pub struct Struct {
    pub name: String,
    pub json_type: JSONType,
    pub r#type: Type
}

#[derive(Deserialize)]
pub struct StructField {
    pub name: String,
    pub json_type: JSONType,
    pub r#type: Type,
    pub bitsize: u8
}

#[derive(Deserialize)]
pub struct Union {
    pub name: String,
    pub json_type: JSONType,
    pub r#type: Type
}

#[derive(Deserialize)]
pub struct Typedef {
    pub name: String,
    pub json_type: JSONType,
    pub r#type: Type
}

pub fn parse(source_str: &str) -> Root {
    todo!()
}
