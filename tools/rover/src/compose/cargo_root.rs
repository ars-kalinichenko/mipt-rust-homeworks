use super::config::Config;
use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn cargo_root(output: &Path, config: &Config) -> Result<()> {
    let problems: Vec<_> = config
        .get_problems()
        .iter()
        .map(|path| {
            let path = PathBuf::from("problems").join(path);
            path.to_str().unwrap().to_string()
        })
        .collect();
    let tools: Vec<_> = config
        .get_tools()
        .iter()
        .map(|path| {
            let path = PathBuf::from("tools").join(path);
            path.to_str().unwrap().to_string()
        })
        .collect();
    let add_to_toml: Vec<_> = config
        .get_add_to_toml()
        .iter()
        .map(|path| path.to_str().unwrap().to_string())
        .collect();
    let content = format!(
        r#"[workspace]
members = [
    # Problems
    "{}",

    # Tools
    "{}",

    # Additional
    "{}",
]
"#,
        problems.join("\",\n    \""),
        tools.join("\",\n    \""),
        add_to_toml.join("\",\n    \"")
    );
    fs::write(output.join("Cargo.toml"), content).context("failed to write Cargo.toml")
}
