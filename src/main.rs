extern crate clap;
extern crate bulk_renamer;

use std::path::PathBuf;
use std::process;
use clap::Parser;
use regex::Regex;
use bulk_renamer::bulk_renamer::bulk_rename;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Set the directory to perform a bulk rename.
    #[arg(short, long)]
    dir: PathBuf,

    /// Set the from regex.
    #[arg(short, long)]
    from: String,

    /// Set the to regex.
    #[arg(short, long)]
    to: String,

    /// Perform a dry-run.
    #[arg(short = 'r', long, default_value_t = false)]
    dry_run: bool,
}

fn err_and_exit(message: &str) {
    eprintln!("Error: {}", message);
    process::exit(1);
}

fn main() {
    let args = Args::parse();
    if !args.dir.is_dir() {
        err_and_exit(&format!("{:?} is not a directory.", args.dir.as_path()));
    }
    let from = Regex::new(&args.from);
    if from.is_err() {
        err_and_exit(&format!("Invalid from regex '{}'", args.from));
    }
    let to = Regex::new(&args.to);
    if to.is_err() {
        err_and_exit(&format!("Invalid to regex '{}'", args.to));
    }
    bulk_rename(args.dir.as_path(), from.unwrap(), to.unwrap(), args.dry_run);
}
