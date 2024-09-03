# bmv
A CLI to do a bulk rename.

## Usage

### API
```rust
struct SimpleCallback {}

impl Callback for SimpleCallback {
    fn on_ok(&self, old_path: &Path, new_path: &Path) {
        println!("OK: {} --> {}", old_path.display(), new_path.display());
    }

    fn on_error(&self, old_path: &Path, new_path: &Path, error: std::io::Error) {
        eprintln!(
            "Error: Unable to rename {} to {}: {}",
            old_path.display(),
            new_path.display(),
            error
        );
    }
}

match BulkRename::new(path, r"(test)_(\d+).txt", r"${2}_${1}.txt") {
    Ok(br) => {
        br.bulk_rename(NoOpCallback::new());
    }
    Err(e) => {
        eprintln!("Error: {:?}", e);
    }
}
```

### CLI
```
Usage: bmv [OPTIONS] --dir <DIR> --regex <REGEX> --replacement <REPLACEMENT>

Options:
  -f, --dir <DIR>                  Set the directory
  -r, --regex <REGEX>              Set the regex
  -p, --replacement <REPLACEMENT>  Set the replacement
  -d, --dry-run                    Perform a dry-run
  -q, --quiet                      Run in quiet mode
  -h, --help                       Print help
  -V, --version                    Print version
```
