use super::config::Config;
use anyhow::{Context, Result};
use std::{fs, path::Path};

pub fn skip(output: &Path, config: &Config) -> Result<()> {
    for entry in config.get_skipped() {
        let path = output.join(entry);
        if path.is_dir() {
            fs::remove_dir_all(&path)
        } else {
            fs::remove_file(&path)
        }
        .with_context(|| format!("failed to remove {path:?}"))?;
    }
    Ok(())
}
