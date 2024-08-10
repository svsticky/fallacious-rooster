use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct AppArgs {
    pub config: PathBuf,
}
