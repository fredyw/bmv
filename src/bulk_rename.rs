use rayon::prelude::*;
use regex::Regex;
use std::path::Path;
use std::{fs, io};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct BulkRename<'a> {
    dir: &'a Path,
    regex: Regex,
    replacement: &'a str,
}

pub enum Error {
    NotDirError,
    RegexError(regex::Error),
}

pub trait Callback: Sync + Send {
    fn on_ok(&self, old_path: &Path, new_path: &Path);
    fn on_error(&self, old_path: &Path, new_path: &Path, error: io::Error);
}

impl<'a> BulkRename<'a> {
    pub fn new(dir: &'a Path, regex: &'a str, replacement: &'a str) -> Result<Self, Error> {
        if !dir.is_dir() {
            return Err(Error::NotDirError);
        }
        let regex = Regex::new(regex).map_err(|e| Error::RegexError(e))?;
        Ok(Self {
            dir,
            regex,
            replacement,
        })
    }

    pub fn bulk_rename_fn<F>(&self, f: F)
    where
        F: Fn(&Path, &Path) + Sync + Send,
    {
        WalkDir::new(self.dir)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .par_bridge()
            .for_each(|entry| {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name() {
                        if let Some(old_file_name) = file_name.to_str() {
                            let new_file_name = self
                                .regex
                                .replace_all(old_file_name, self.replacement)
                                .to_string();
                            let mut new_path = path.to_path_buf();
                            new_path.set_file_name(new_file_name);
                            f(&path, &new_path);
                        }
                    }
                }
            });
    }

    pub fn bulk_rename(&self, callback: impl Callback) {
        self.bulk_rename_fn(|old_path, new_path| match fs::rename(old_path, new_path) {
            Ok(_) => {
                callback.on_ok(old_path, new_path);
            }
            Err(error) => {
                callback.on_error(old_path, new_path, error);
            }
        })
    }
}
