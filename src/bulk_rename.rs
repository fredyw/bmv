use rayon::prelude::*;
use regex::Regex;
use std::path::Path;
use std::{fs, io};
use walkdir::WalkDir;

pub enum BulkRenameError {
    NotDirError,
    RegexError(regex::Error),
}

pub fn bulk_rename_fn(
    dir: &Path,
    regex: &str,
    replacement: &str,
    fun: fn(&Path, &Path) -> Result<(), io::Error>,
) -> Result<(), BulkRenameError> {
    if !dir.is_dir() {
        return Err(BulkRenameError::NotDirError);
    }
    let regex = Regex::new(regex).map_err(|e| BulkRenameError::RegexError(e))?;
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .par_bridge()
        .for_each(|entry| {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(old_file_name) = file_name.to_str() {
                        let new_file_name =
                            regex.replace_all(old_file_name, replacement).to_string();
                        let mut new_path = path.to_path_buf();
                        new_path.set_file_name(new_file_name);
                        // TODO: Don't unwrap.
                        fun(&path, &new_path).unwrap();
                    }
                }
            }
        });
    Ok(())
}

pub fn bulk_rename(dir: &Path, from: &str, to: &str) -> Result<(), BulkRenameError> {
    bulk_rename_fn(dir, from, to, |old_path, new_path| {
        fs::rename(old_path, new_path)
    })
}
