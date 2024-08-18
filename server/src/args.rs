use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct AppArgs {
    #[clap(long, short)]
    pub config: PathBuf,
}
