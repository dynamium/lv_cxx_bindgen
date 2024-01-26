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
    /// Changes how anonymous enums (that is, enums without names) are handled.
    #[arg(long, value_enum, default_value_t = AnonEnumGeneration::Infer)]
    pub anon_enum_handling: AnonEnumGeneration
}

#[derive(Debug, Clone, ValueEnum)]
pub enum AnonEnumGeneration {
    /// Converts for example LV_PROPERTY_ID_INVALID to lvgl::property_id_invalid
    Constexpr,
    /// For example, LV_PROPERTY_ID_INVALID becomes lvgl::PropertyID::Invalid
    Infer
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
