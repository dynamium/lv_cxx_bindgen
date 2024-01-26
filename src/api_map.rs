use anyhow::Result;
use itertools::Itertools;
use log::{debug, trace};
use serde::{ser::SerializeMap, Deserialize};

#[derive(Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct JSONValue {
    pub name: Option<String>,
    pub json_type: JSONType,
    pub docstring: Option<String>,
    pub quals: Option<Vec<String>>,
    pub storage: Option<Vec<String>>,
    pub r#type: Option<Box<JSONValue>>, // wrapped in a Box to fix type recursion
    pub fields: Option<Vec<JSONValue>>,
    pub members: Option<Vec<JSONValue>>,
    pub args: Option<Vec<JSONValue>>,
    pub bitsize: Option<String>,
}

impl JSONValue {
    fn parse_as_type(&self) -> String {
        if self.r#type.clone().unwrap().name.is_none() {
            return format!("{}*", self.r#type.clone().unwrap().parse_as_type());
        }

        return self.r#type.clone().unwrap().name.unwrap();
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum JSONType {
    PrimitiveType,
    StdlibType,
    #[serde(rename = "lvgl_type")]
    LVGLType,
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
    FunctionPointer,
    Variable,
    Union,
    #[serde(rename = "forward_decl")]
    ForwardDeclaration,
    Macro,
    Arg,
    SpecialType,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct APIMap {
    pub enums: Vec<Enum>,
    pub functions: Vec<Function>,
    pub structs: Vec<Struct>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Enum {
    pub identifier: Option<String>,
    pub members: Vec<EnumMember>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct EnumMember {
    pub identifier: String,
    /// When enums are processed, their names get changed to be more
    /// C++-like. This field is for storing the original name of the member.
    // pub original: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Function {
    pub identifier: String,
    pub return_type: String,
    pub args: Vec<FuncArg>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FuncArg {
    pub identifier: Option<String>,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Struct {
    pub identifier: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StructField {
    pub identifier: String,
    pub kind: String,
    pub bitsize: Option<u8>,
}

// god forgive me for the amount of .unwrap() statements that
// there will be in this function
pub fn parse(source_str: &str) -> Result<APIMap> {
    let json: JSONRoot = serde_json::from_str(source_str)?;

    Ok(APIMap {
        enums: json.process_enums(),
        functions: json.process_functions(),
        structs: json.process_structs(),
    })
}

impl JSONRoot {
    fn process_enums(&self) -> Vec<Enum> {
        let typedef_enums = self
            .typedefs
            .clone()
            .into_iter()
            .filter(|td| {
                td.r#type.clone().unwrap().json_type == JSONType::Enum
                    && td.r#type.clone().unwrap().name == None
            })
            .map(|typedef| Enum {
                identifier: typedef.name,
                members: typedef
                    .r#type
                    .unwrap()
                    .members
                    .unwrap()
                    .into_iter()
                    .map(|m| EnumMember {
                        identifier: m.name.unwrap(),
                        value: None,
                    })
                    .collect(),
            });

        let flat_typedefs = self
            .typedefs
            .clone()
            .into_iter()
            .filter(|td| {
                td.r#type.as_ref().unwrap().fields.is_none()
                    && td.r#type.as_ref().unwrap().json_type == JSONType::LVGLType
            })
            .collect::<Vec<_>>();

        self.enums
            .clone()
            .into_iter()
            .map(|item| Enum {
                identifier: item.name,
                members: item
                    .members
                    .unwrap()
                    .into_iter()
                    .map(|member| EnumMember {
                        identifier: member.name.unwrap(),
                        value: None,
                    })
                    .collect(),
            })
            .map(move |item| {
                trace!("{item:#?}");
                if item.identifier.is_none() {
                    return item;
                }

                if let Some(td) = flat_typedefs.clone().into_iter().find(|i| {
                    trace!("{i:#?}");
                    i.name.as_ref().unwrap() == (&item).identifier.as_ref().unwrap()
                }) {
                    trace!("Found typedef: {td:#?}");
                    let mut item = item.clone();
                    item.identifier = td.r#type.unwrap().name;
                    return item;
                }

                return item;
            })
            .merge(typedef_enums)
            .collect::<Vec<_>>()
    }

    fn process_functions(&self) -> Vec<Function> {
        self.functions
            .clone()
            .into_iter()
            .map(|func| {
                let func = func.clone();
                Function {
                    identifier: func.name.clone().unwrap(),
                    return_type: func.r#type.clone().unwrap().parse_as_type(),
                    args: func
                        .clone()
                        .args
                        .unwrap_or(vec![])
                        .into_iter()
                        .map(|arg| FuncArg {
                            identifier: arg.name.clone(),
                            kind: arg.parse_as_type(),
                        })
                        .filter(|arg| {
                            if func.args.clone().unwrap_or(vec![]).len() == 1
                                && arg.identifier.is_none()
                                && arg.kind == "void"
                            {
                                return false;
                            }
                            true
                        })
                        .collect(),
                }
            })
            .collect()
    }

    fn process_structs(&self) -> Vec<Struct> {
        let typedefs: Vec<_> = self
            .typedefs
            .clone()
            .into_iter()
            .filter(|td| td.r#type.clone().unwrap().json_type == JSONType::LVGLType)
            .collect();
        self.structures
            .clone()
            .into_iter()
            .map(|structure| Struct {
                identifier: ident_from_typedefs(&typedefs, &structure.name.unwrap()),
                fields: structure
                    .fields
                    .unwrap()
                    .into_iter()
                    .map(|field| StructField {
                        identifier: field.clone().name.unwrap(),
                        // I know that this might be redudant, but just to be safe, if
                        // *maybe* a non-typedef type is used, idk
                        kind: ident_from_typedefs(&typedefs, &field.parse_as_type()),
                        bitsize: field.bitsize.map(|x| x.parse().unwrap()),
                    })
                    .collect(),
            })
            .collect()
    }
}

// TODO: Document
fn ident_from_typedefs(typedefs: &[JSONValue], ident: &str) -> String {
    let (type_modifiers, clean_ident) = {
        let mut res = String::new();
        for char in ident.chars() {
            if char == '*' {
                res.push(char);
            }
        }
        (res, ident.chars().filter(|c| *c != '*').collect::<String>())
    };
    let typedef = typedefs
        .into_iter()
        .find(|td| td.r#type.clone().unwrap().name.unwrap() == clean_ident);
    if let Some(typedef) = typedef {
        let mut ident = typedef.name.clone().unwrap_or(ident.to_string());
        ident.push_str(&type_modifiers);
        return ident;
    }
    return ident.to_string();
}
