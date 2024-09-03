use rayon::prelude::*;
use regex::Regex;
use std::path::Path;
use walkdir::WalkDir;

pub fn bulk_rename(dir: &Path, from: Regex, to: Regex, dry_run: bool) {
    let walker = WalkDir::new(dir).into_iter();
    walker
        .filter_map(|entry| entry.ok())
        .par_bridge()
        .for_each(|entry| {
            let path = entry.path();
            if path.is_file() {
                println!("{:?}", path);
            }
        });
}
