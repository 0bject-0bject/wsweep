/// 
/// Copyright 2023, [object Object]
/// Licensed under MIT
///
/// Warnings:
/// Please check every file before deleting it, i am not responsible for any damage caused by this tool!
/// 

use std::{path::PathBuf, env, io::{Error, ErrorKind}};
use clap::{Command, Arg};

pub struct CliArgs {
    pub path: PathBuf,
    pub age: u64,
    pub dry_run: bool,
    pub verbose: bool,
}

// Parse the CLI arguments and return a `CliArgs` struct
pub fn parse_cli() -> std::io::Result<CliArgs> {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("[object Object] <whaledev.contact@gmail.com>")
        .about("A simple tool to clear out node_modules and cargo target directories")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("path")
                .required(false)
                .help("Path to start search at."),
        )
        .arg(
            Arg::new("age")
                .long("age")
                .value_name("age")
                .required(false)
                .help("Minimum age for file to be deleted. (DAYS)"),
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .value_name("dry-run")
                .required(false)
                .num_args(0)
                .help("Run without deleting files."),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .value_name("verbose")
                .required(false)
                .num_args(0)
                .help("Verbose output."),
        )
        .get_matches();

    let path = matches
        .get_one::<String>("path")
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap());

    let age = matches
        .get_one::<String>("age")
        .unwrap_or(&"0".to_string())
        .parse::<u64>()
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid age value"))?;

    let dry_run = matches
        .get_one::<bool>("dry-run")
        .unwrap_or(&false);

    let verbose = matches
        .get_one::<bool>("verbose")
        .unwrap_or(&false);

    Ok(CliArgs { path, age, dry_run: *dry_run, verbose: *verbose })
}