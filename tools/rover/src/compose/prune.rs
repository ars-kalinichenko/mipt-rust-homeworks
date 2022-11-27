use super::config::Config;
use anyhow::{Context, Result};
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

pub fn prune(output: &Path, config: &Config) -> Result<()> {
    let dir = fs::read_dir(output);
    if dir.is_err() {
        return Ok(());
    }
    let dir = dir.unwrap();
    let do_not_delete: HashSet<_> = config.get_do_not_delete().iter().collect();
    let output_len = output.iter().count();
    for entry in dir {
        let path = entry.context("cannot read entry")?.path();
        let relative: PathBuf = path.iter().skip(output_len).collect();
        if !do_not_delete.contains(&relative) {
            if path.is_dir() {
                fs::remove_dir_all(&path)
            } else {
                fs::remove_file(&path)
            }
            .with_context(|| format!("failed to remove {path:?}"))?;
        }
    }
    Ok(())
}
