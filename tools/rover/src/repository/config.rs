#![allow(clippy::needless_question_mark)]

use super::{command::Command, context::CommandContext, step::Step, toolchain::Toolchain};
use anyhow::{Context, Result};
use glob::{glob_with, MatchOptions};
use serde_yaml::{from_reader, Value};
use std::{
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Config {
    workdir: PathBuf,
    toolchain: Toolchain,
    relative_user_files: Vec<PathBuf>,
    absolute_user_files: Vec<PathBuf>,
    steps: Vec<Step>,
}

impl Config {
    pub fn from_yml(path: &Path) -> Result<Self> {
        let file = File::open(path).context("no yml file with config")?;
        let yml: Value = from_reader(file).context("unable to read yml file with config")?;
        let yml = yml.as_mapping().context("the yml is not a mapping")?;
        let workdir = path.parent().context("yml has no parent")?.to_path_buf();
        let toolchain = Toolchain::from_name(
            yml[&Value::String("toolchain".to_string())]
                .as_str()
                .context("toolchain is not a string")?,
        )?;
        let patterns = yml[&Value::String("allowed-patterns".to_string())]
            .as_sequence()
            .context("user files are not in a list")?
            .iter()
            .map(|value| Ok(value.as_str().context("user file path is not a string")?))
            .collect::<Result<Vec<_>>>()?;
        let (relative_user_files, absolute_user_files) =
            Self::get_matching_user_files(&workdir, patterns.as_slice())?;
        let steps = yml[&Value::String("steps".to_string())]
            .as_mapping()
            .context("steps is not a mapping")?
            .iter()
            .map(|(key, value)| {
                let name = key
                    .as_str()
                    .context("name of the step is not a string")?
                    .to_string();
                let commands = value
                    .as_sequence()
                    .context("the step commands are not a sequence")?
                    .iter()
                    .map(|value| Command::from_name(value.as_str().unwrap()).unwrap())
                    .collect();
                Ok(Step::new(name, commands))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            workdir,
            toolchain,
            relative_user_files,
            absolute_user_files,
            steps,
        })
    }

    pub fn get_steps(&self) -> &[Step] {
        self.steps.as_slice()
    }

    pub fn get_toolchain(&self) -> &Toolchain {
        &self.toolchain
    }

    pub fn get_relative_user_files(&self) -> &[PathBuf] {
        self.relative_user_files.as_slice()
    }

    #[allow(dead_code)]
    pub fn get_absolute_user_files(&self) -> &[PathBuf] {
        self.absolute_user_files.as_slice()
    }

    pub fn get_command_context(&self) -> CommandContext {
        CommandContext::new(&self.workdir, self.absolute_user_files.as_slice())
    }

    fn get_matching_user_files(
        workdir: &Path,
        patterns: &[&str],
    ) -> Result<(Vec<PathBuf>, Vec<PathBuf>)> {
        let options = MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };
        let workdir_len = workdir.iter().count();
        let mut relative_user_files = Vec::new();
        let mut absolute_user_files = Vec::new();
        for pattern in patterns {
            let pattern = workdir.join(pattern).to_path_buf();
            let pattern = pattern.to_str().context("non-utf-8 path")?;
            for entry in glob_with(pattern, options).context("pattern is invalid")? {
                let entry = entry?;
                absolute_user_files.push(entry.clone());
                relative_user_files.push(entry.iter().skip(workdir_len).collect());
            }
        }
        Ok((relative_user_files, absolute_user_files))
    }
}
