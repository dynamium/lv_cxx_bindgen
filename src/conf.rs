use std::path::PathBuf;

use clap::ValueEnum;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub input: Input,
    pub generation: Generation,
}

#[derive(Deserialize, Debug)]
pub struct Input {
    pub cwd: Option<PathBuf>,
}

#[derive(Deserialize, Debug)]
pub struct Generation {
    pub target: CxxVersion,
    pub class_exclude: Vec<GenerationClassItem>,
    pub namespace_exclude: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, ValueEnum)]
pub enum CxxVersion {
    #[serde(rename = "c++11")]
    #[clap(name = "c++11")]
    Cxx11,
    #[serde(rename = "c++14")]
    #[clap(name = "c++14")]
    Cxx14,
    #[serde(rename = "c++17")]
    #[clap(name = "c++17")]
    Cxx17,
    #[serde(rename = "c++20")]
    #[clap(name = "c++20")]
    Cxx20,
    #[serde(rename = "c++23")]
    #[clap(name = "c++23")]
    Cxx23,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GenerationClassItem {
    Simple(String),
    Full(GenClass),
}

#[derive(Deserialize, Debug)]
pub struct GenClass {
    pub ident: String,
    pub inherits: Vec<String>,
}

pub struct Output {
    pub path: PathBuf,
}
