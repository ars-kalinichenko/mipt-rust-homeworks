use super::{cargo_root::cargo_root, process::process, prune::prune, skip::skip};
use crate::repository::repo::Repository;
use anyhow::{Context, Result};
use std::path::Path;

pub fn run_compose(input: &Path, output: &Path) -> Result<()> {
    let repository = Repository::from_path(input)?;
    let config = repository.compose_config()?;
    let input = repository.get_path().to_path_buf();
    let output = output
        .parent()
        .context("output path has no parent to canonicalize")?
        .canonicalize()
        .context("failed to canonicalize output path")?
        .join(
            output
                .file_name()
                .context("output path has no file name to canonicalize")?,
        );
    prune(&output, &config)?;
    process(&input, &output, &config)?;
    cargo_root(&output, &config)?;
    skip(&output, &config)
}
