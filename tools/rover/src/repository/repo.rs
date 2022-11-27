use super::problem::Problem;
use crate::compose;
use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

pub const COMPOSE_CONFIG: &str = ".compose.yml";
pub const PROBLEMS_FOLDER: &str = "problems";
pub const SOLUTIONS_REPO_FOLDER: &str = "solutions";
pub const REPOSITORY_NAMES: [&str; 2] = ["mipt-rust", "mipt-rust-private"];

pub struct Repository {
    path: PathBuf,
}

impl Repository {
    pub fn from_path(path: &Path) -> Result<Self> {
        let path = path
            .canonicalize()
            .context("cannot canonicalize path for repository")?;
        let prefix_count = path
            .iter()
            .position(|comp| REPOSITORY_NAMES.iter().any(|name| *name == comp));
        if let Some(pos) = prefix_count {
            Ok(Self {
                path: path.iter().take(pos + 1).collect(),
            })
        } else {
            bail!("path does not contain course repository")
        }
    }

    pub fn problem_from_path(&self, path: &Path) -> Result<Problem> {
        let path = path
            .canonicalize()
            .context("failed to canonicalize path for problem")?;
        let title = path
            .file_name()
            .context("the path to problem does not contain problem name")?
            .to_str()
            .unwrap()
            .to_string();
        let group = path
            .iter()
            .rev()
            .nth(1)
            .context("the path to problem does not contain problem group")?
            .to_str()
            .unwrap()
            .to_string();
        if self.path.join(PROBLEMS_FOLDER).join(group).join(title) == path {
            Ok(Problem::from_path(&path))
        } else {
            bail!("problem path is not in REPOSITORY/problems/GROUP/TITLE")
        }
    }

    pub fn solutions_repo(&self) -> Result<PathBuf> {
        let path = self.path.parent().unwrap().join(SOLUTIONS_REPO_FOLDER);
        if path.is_dir() {
            Ok(path)
        } else {
            bail!("solutions repository does not exist")
        }
    }

    pub fn compose_config(&self) -> Result<compose::config::Config> {
        compose::config::Config::from_yml(&self.path.join(COMPOSE_CONFIG))
    }

    #[allow(unused)]
    pub fn get_path(&self) -> &Path {
        &self.path
    }
}
