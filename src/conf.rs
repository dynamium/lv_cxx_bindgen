use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub input: ConfigInput,
    pub generation: ConfigGen,
}

#[derive(Deserialize, Debug)]
pub struct ConfigInput {
    pub cwd: String,
    pub files: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct ConfigGen {
    pub classes: Vec<String>,
}
