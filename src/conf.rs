use std::{path::PathBuf};

use clap::ValueEnum;
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
    pub target: CxxVersion,
    pub class_exclude: Vec<ConfigGenClassItem>,
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

pub struct ConfigOutput {
    pub path: PathBuf,
}
