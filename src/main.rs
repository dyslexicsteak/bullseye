mod cli;

use std::{
    fs::{metadata, read_dir},
    io,
    process::Command,
};

use cli::{Cli, Output};

use clap::Parser;
use duration_str::parse;
use rayon::iter::ParallelBridge;
use rayon::prelude::*;

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let duration = args.duration.unwrap_or("1w".into());

    let duration = parse(&duration).unwrap();
    let verbose = matches!(args.output, Some(Output::Verbose));
    let dry_run = args.dry_run;

    let path = args.directory.canonicalize()?;

    if !path.is_dir() {
        panic!("The directory argument must be a path that points at a directory!");
    }

    let dir = read_dir(path)?;

    dir.into_iter().par_bridge().for_each(|entry| {
        let entry = entry.unwrap();
        let mut path = entry.path();

        let metadata = metadata(&path).unwrap();
        let last_modified = metadata
            .modified()
            .expect("Unable to get last modified time.")
            .elapsed()
            .expect("Failed to get elapsed time.");

        if last_modified >= duration {
            path.push("Cargo.toml");

            if path.exists() {
                if !dry_run {
                    Command::new("cargo")
                        .arg("clean")
                        .arg("--manifest-path")
                        .arg(&path)
                        .arg("-q")
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap();
                }

                if verbose {
                    println!("Project at {path:#?} cleaned.");
                }
            }
        }
    });

    Ok(())
}
