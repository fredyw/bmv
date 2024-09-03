extern crate bmv;

use bmv::{BulkRename, Error};
use std::path::Path;

#[test]
fn path_is_not_a_directory() {
    let bulk_rename = BulkRename::new(Path::new("doesntexist"), "foo", "bar");
    assert!(matches!(bulk_rename.unwrap_err(), Error::NotDirError));
}

#[test]
fn regex_is_invalid() {
    let bulk_rename = BulkRename::new(Path::new("."), r"(\d+", "bar");
    assert!(matches!(bulk_rename.unwrap_err(), Error::RegexError(_)));
}
