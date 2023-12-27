use clap::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long)]
    pub verbose: bool,
}
