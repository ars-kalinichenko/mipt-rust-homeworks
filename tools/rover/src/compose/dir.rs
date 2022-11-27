use super::file::process_file;
use anyhow::{Context, Result};
use std::{fs, path::Path};

pub fn process_dir(input: &Path, output: &Path) -> Result<()> {
    let dir = fs::read_dir(input).with_context(|| format!("failed to read directory {input:?}"))?;
    for entry in dir {
        let input = entry
            .with_context(|| format!("failed to read entry in directory {input:?}"))?
            .path();
        let output = output.join(input.file_name().unwrap());
        if input.is_dir() {
            process_dir(&input, &output)?;
        } else {
            process_file(&input, &output)?;
        }
    }
    Ok(())
}
