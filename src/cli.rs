use std::path::PathBuf;

use clap::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
    #[arg(short, long, default_value = "lv_cxx_bindgen.toml")]
    pub config: PathBuf,
}
