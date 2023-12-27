use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub input_files: Vec<String>,
    pub gen: ConfigGen,
}

#[derive(Deserialize, Debug)]
pub struct ConfigGen {
    pub classes: Vec<String>,
}
