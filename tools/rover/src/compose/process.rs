use super::{config::Config, dir::process_dir, file::process_file};
use anyhow::Result;
use std::path::{Path, PathBuf};

pub fn process(input: &Path, output: &Path, config: &Config) -> Result<()> {
    let to_process = std::iter::empty()
        .chain(
            config
                .get_problems()
                .iter()
                .map(|path| PathBuf::from("problems").join(path)),
        )
        .chain(
            config
                .get_tools()
                .iter()
                .map(|path| PathBuf::from("tools").join(path)),
        )
        .chain(config.get_copy().iter().cloned());
    for entry in to_process {
        let input = input.join(&entry);
        let output = output.join(&entry);
        if input.is_dir() {
            process_dir(&input, &output)?;
        } else {
            process_file(&input, &output)?;
        }
    }
    Ok(())
}
