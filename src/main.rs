extern crate bmv;
extern crate clap;

use bmv::bulk_rename::BulkRename;
use bmv::bulk_rename::Callback;
use bmv::bulk_rename::Error;
use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Set the directory.
    #[arg(short = 'f', long)]
    dir: PathBuf,

    /// Set the regex.
    #[arg(short = 'r', long)]
    regex: String,

    /// Set the replacement.
    #[arg(short = 'p', long)]
    replacement: String,

    /// Perform a dry-run.
    #[arg(short = 'd', long, default_value_t = false)]
    dry_run: bool,

    /// Run the bulk rename in quiet mode.
    #[arg(short = 'q', long, default_value_t = false)]
    quiet: bool,
}

struct CliCallback {
    quiet: bool,
}

impl CliCallback {
    fn new(quiet: bool) -> Self {
        Self { quiet }
    }
}

impl Callback for CliCallback {
    fn on_ok(&self, old_path: &Path, new_path: &Path) {
        if !self.quiet {
            println!("OK: {} --> {}", old_path.display(), new_path.display());
        }
    }

    fn on_error(&self, old_path: &Path, new_path: &Path, error: std::io::Error) {
        if !self.quiet {
            eprintln!(
                "Error: Unable to rename {} to {}: {}",
                old_path.display(),
                new_path.display(),
                error
            );
        }
    }
}

fn main() {
    let args = Args::parse();
    let path = args.dir.as_path();
    let bulk_rename = BulkRename::new(path, &args.regex, &args.replacement);
    let result = if args.dry_run {
        bulk_rename.bulk_rename_fn(|old_path, new_path| {
            println!("{} --> {}", old_path.display(), new_path.display());
        })
    } else {
        bulk_rename.bulk_rename(CliCallback::new(args.quiet))
    };
    match result {
        Ok(_) => {}
        Err(error) => match error {
            Error::NotDirError => {
                eprintln!("Error: {} is not a directory", path.display())
            }
            Error::RegexError(error) => {
                eprintln!("Error: {} is not a valid regex: '{}'", args.regex, error)
            }
        },
    }
}
