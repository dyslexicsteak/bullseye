use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version)]
pub(crate) struct Cli {
    pub(crate) directory: PathBuf,
    pub(crate) duration: Option<String>,
}
