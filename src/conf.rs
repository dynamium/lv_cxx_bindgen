use std::path::PathBuf;

use clap::ValueEnum;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub input: Input,
    pub generation: Generation,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Input {
    pub cwd: Option<PathBuf>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Generation {
    pub functions: FunctionsConfig,
    pub classes: ClassesConfig,
    pub namespaces: NamespacesConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ClassesConfig {
    #[serde(default)]
    pub exclude: Vec<ExcludeInclude>,
    #[serde(default)]
    pub include: Vec<ExcludeInclude>,
    #[serde(default)]
    pub rename: Vec<(String, String)>,
    #[serde(default)]
    pub inherit: Vec<(String, String)>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FunctionsConfig {
    #[serde(default)]
    pub exclude: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NamespacesConfig {
    #[serde(default)]
    pub exclude: Vec<ExcludeInclude>,
    #[serde(default)]
    pub include: Vec<ExcludeInclude>,
    #[serde(default)]
    pub rename: Vec<(String, String)>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExcludeInclude {
    #[serde(default)]
    pub namespaces: Vec<String>,
    #[serde(default)]
    pub functions: Vec<String>,
    #[serde(default)]
    pub types: Vec<String>,
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

pub struct Output {
    pub path: PathBuf,
}
