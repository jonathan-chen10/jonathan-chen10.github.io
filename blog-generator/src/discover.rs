use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub fn discover(input_dir: &Path) -> Vec<PathBuf> {
    WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|e| e.ok()) // silently skips error
        .filter(|e| e.path().extension() == Some(OsStr::new("md")))
        .map(|e| e.path().to_path_buf())
        .collect()
}
