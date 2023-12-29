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
    pub classes: Vec<String>,
    pub namespaces: Vec<String>,
}
