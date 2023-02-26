mod cli;

use std::{
    fs::{metadata, read_dir},
    io,
    process::Command,
};

use cli::Cli;

use clap::Parser;
use duration_str::parse;

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let duration = args.duration.unwrap_or("1w".into());

    let duration = parse(&duration).unwrap();

    let path = args.directory.canonicalize()?;

    if !path.is_dir() {
        panic!("The directory argument must be a path that points at a directory!");
    }

    let dir = read_dir(path)?;

    for entry in dir {
        let entry = entry?;
        let mut path = entry.path();

        let metadata = metadata(&path)?;
        let last_modified = metadata
            .modified()?
            .elapsed()
            .expect("Failed to get elapsed time.");

        if !(last_modified >= duration) {
            continue;
        }

        path.push("Cargo.toml");

        if path.exists() {
            Command::new("cargo")
                .arg("clean")
                .arg("--manifest-path")
                .arg(&path)
                .arg("-q")
                .spawn()?
                .wait()?;
            dbg!(path);
        }
    }

    Ok(())
}
