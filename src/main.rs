extern crate bmv;
extern crate clap;

use bmv::bulk_rename;
use bmv::bulk_rename::BulkRenameError;
use bmv::bulk_rename_fn;
use clap::Parser;
use std::path::PathBuf;

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
}

fn main() {
    let args = Args::parse();
    let path = args.dir.as_path();
    let result = if args.dry_run {
        bulk_rename_fn(
            path,
            &args.regex,
            &args.replacement,
            |old_path, new_path| {
                println!("{} --> {}", old_path.display(), new_path.display());
                Ok(())
            },
        )
    } else {
        bulk_rename(path, &args.regex, &args.replacement)
    };
    match result {
        Ok(_) => {}
        Err(e) => match e {
            BulkRenameError::NotDirError => {
                eprintln!("Error: {} is not a directory", path.display())
            }
            BulkRenameError::RegexError(e) => {
                eprintln!("Error: {} is not a valid regex: '{}'", args.regex, e)
            }
        },
    }
}
