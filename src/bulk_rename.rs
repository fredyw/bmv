use rayon::prelude::*;
use regex::Regex;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub enum BulkRenameError {
    NotDirError,
    RegexError(regex::Error),
}

pub fn bulk_rename_fn<F: Fn(&Path, &Path) + Sync + Send>(
    dir: &Path,
    regex: &str,
    replacement: &str,
    f: F,
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
                        f(&path, &new_path);
                    }
                }
            }
        });
    Ok(())
}

pub fn bulk_rename(dir: &Path, regex: &str, replacement: &str) -> Result<(), BulkRenameError> {
    bulk_rename_fn(
        dir,
        regex,
        replacement,
        |old_path, new_path| match fs::rename(old_path, new_path) {
            Ok(_) => {}
            Err(_) => {}
        },
    )
}
