extern crate clap;

use std::path::PathBuf;
use std::process;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Set the directory to perform a bulk rename.
    #[arg(short, long)]
    dir: PathBuf,

    /// Set the from regex pattern.
    #[arg(short, long)]
    from: String,

    /// Set the to regex pattern.
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
}
