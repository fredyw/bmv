use rayon::prelude::*;
use regex::Regex;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn bulk_rename_fn(dir: &Path, from: &str, to: &str, fun: fn(&Path, &Path)) {
    // TODO: don't unwrap()
    let regex = Regex::new(from).unwrap();
    let walker = WalkDir::new(dir).into_iter();
    walker
        .filter_map(|entry| entry.ok())
        .par_bridge()
        .for_each(|entry| {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(old_file_name) = file_name.to_str() {
                        let new_file_name = regex.replace_all(old_file_name, to).to_string();
                        let mut new_path = path.to_path_buf();
                        new_path.set_file_name(new_file_name);
                        fun(&path, &new_path);
                    }
                }
            }
        });
}

pub fn bulk_rename(dir: &Path, from: &str, to: &str) {
    bulk_rename_fn(dir, from, to, |old_path, new_path| {
        // TODO: don't unwrap()
        fs::rename(old_path, new_path).unwrap();
    });
}
