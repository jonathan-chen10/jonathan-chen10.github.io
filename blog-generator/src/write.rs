use std::{fs, io};
use std::path::{Path};

use anyhow::{Context, Result};

use crate::types::OutputFile;

/// Filesystem-level write. Writes a single file.
/// If blog, also copies assets.
pub fn write(files: Vec<OutputFile>, output_dir: &Path) -> Result<()> {
    // clear output dir
    match fs::remove_dir_all(output_dir) {
        Ok(_) => {},
        Err(e) if e.kind() == io::ErrorKind::NotFound => {},
        Err(e) => return Err(e.into()),
    }

    for ofile in files {
        let full_path = output_dir.join(&ofile.path_relative);

        // Create parent directories if needed
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Could not create directories for {}", full_path.display()))?;
        }

        fs::write(&full_path, &ofile.html)
            .with_context(|| format!("Could not write to {}", full_path.display()))?;

        if let Some(path_assets) = ofile.path_assets {
            // copy all assets using walkdir
            for entry in walkdir::WalkDir::new(&path_assets) {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|e| e.to_str()) != Some("md") {
                    let relative = path.strip_prefix(&path_assets)?;
                    let dest = output_dir.join(
                        ofile.path_relative
                            .parent().ok_or_else(|| anyhow::anyhow!("Could not get parent of {}", ofile.path_relative.display()))?
                            .join(relative)
                    );
                    fs::create_dir_all(dest.parent().unwrap())?;
                    fs::copy(path, &dest)?;
                }
            }
        }
    }
    
    Ok(())
}
