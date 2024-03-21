use std::path::PathBuf;

use clap::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
    #[arg(short, long, default_value = "lv_cxx_bindgen.toml")]
    pub config: PathBuf,
    #[arg(short, long, value_enum)]
    pub target: Option<CxxVersion>,
    /// Disables replacements of char* with std::string.
    #[arg(long = "no-stdstr", action = ArgAction::SetFalse)]
    pub replace_char: bool,
    /// Disables replacements of function pointers with std::function.
    #[arg(long = "no-stdfunc", action = ArgAction::SetFalse)]
    pub replace_func_pointer: bool,
    /// If set, function/property/constexpr names are generated in camelCase instead of
    /// snake_case.
    #[arg(long)]
    pub use_camel_case: bool,
    /// Changes how anonymous enums (that is, enums without names) are handled. See
    /// examples in expanded help.
    #[arg(long = "anon-enum", value_enum, default_value_t = AnonEnumGeneration::Infer)]
    pub anon_enum_handling: AnonEnumGeneration,
    /// Path to the API map .json file
    #[arg(short, long)]
    pub api_map: String,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum AnonEnumGeneration {
    /// LV_PROPERTY_ID_INVALID becomes lvgl::property_id_invalid
    Constexpr,
    /// LV_PROPERTY_ID_INVALID becomes lvgl::PropertyID::Invalid
    Infer,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CxxVersion {
    #[clap(name = "c++11")]
    Cxx11,
    #[clap(name = "c++14")]
    Cxx14,
    #[clap(name = "c++17")]
    Cxx17,
    #[clap(name = "c++20")]
    Cxx20,
    #[clap(name = "c++23")]
    Cxx23,
}
