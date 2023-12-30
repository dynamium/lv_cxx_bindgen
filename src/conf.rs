use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub input: ConfigInput,
    pub generation: ConfigGen,
}

#[derive(Deserialize, Debug)]
pub struct ConfigInput {
    pub cwd: Option<PathBuf>,
    pub files: Vec<PathBuf>,
    pub auto_scan: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct ConfigGen {
    pub root_namespace: String,
    pub target: CXXVersion,
    pub classes: Vec<ConfigGenClassItem>,
    pub namespaces: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub enum CXXVersion {
    #[serde(rename = "c++11")]
    Cxx11,
    #[serde(rename = "c++17")]
    Cxx17,
    #[serde(rename = "c++20")]
    Cxx20
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ConfigGenClassItem {
    Simple(String),
    Full(ConfigGenClass),
}

#[derive(Deserialize, Debug)]
pub struct ConfigGenClass {
    pub ident: String,
    pub inherits: Vec<String>,
}
