extern crate bmv;
extern crate clap;

use bmv::bulk_rename;
use bmv::bulk_rename_fn;
use clap::Parser;
use std::path::PathBuf;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Set the directory to perform a bulk rename.
    #[arg(short, long)]
    dir: PathBuf,

    /// Set the from regex.
    #[arg(short, long)]
    from: String,

    /// Set the to replacement.
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
    let path = args.dir.as_path();
    if !path.is_dir() {
        err_and_exit(&format!("{:?} is not a directory.", path));
    }
    if args.dry_run {
        bulk_rename_fn(path, &args.from, &args.to, |old_path, new_path| {
            println!("{} --> {}", old_path.display(), new_path.display());
        })
    } else {
        bulk_rename(path, &args.from, &args.to);
    }
}
