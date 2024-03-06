use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
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

pub struct Output {
    pub path: PathBuf,
}
