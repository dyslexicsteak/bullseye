use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(about, version)]
pub(crate) struct Cli {
    pub(crate) directory: PathBuf,
    pub(crate) duration: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<Output>,
    #[arg(long, default_value_t = false)]
    pub(crate) dry_run: bool,
}

#[derive(Clone, Debug, ValueEnum)]
pub(crate) enum Output {
    Quiet,
    Verbose,
}
