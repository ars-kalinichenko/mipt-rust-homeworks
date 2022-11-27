use super::config::Config;
use super::repo::PROBLEMS_FOLDER;
use crate::{launch_git, repository::copying::copy_files};
use anyhow::{bail, Result};
use std::{
    path::{Path, PathBuf},
    process,
};

const DEFAULT_YML_NAME: &str = ".config.yml";

pub struct Problem {
    path: PathBuf,
}

impl Problem {
    pub(super) fn from_path(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }

    pub fn title(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_string()
    }

    pub fn group(&self) -> String {
        self.path
            .iter()
            .rev()
            .nth(1)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn branch_name(&self) -> String {
        self.group() + "/" + &self.title()
    }

    pub fn relative_path(&self) -> PathBuf {
        PathBuf::from(PROBLEMS_FOLDER)
            .join(self.group())
            .join(self.title())
    }

    pub fn config(&self) -> Result<Config> {
        Config::from_yml(&self.path.join(DEFAULT_YML_NAME))
    }

    pub fn launch_all_steps(&self) -> Result<()> {
        let config = self.config()?;
        let toolchain = config.get_toolchain();
        let context = config.get_command_context();
        for step in config.get_steps() {
            for command in step.commands() {
                toolchain.run_command(command, &context)?;
            }
        }
        Ok(())
    }

    pub fn move_solution_files_from(
        &self,
        solutions_repo: &Path,
        checkout_branch: bool,
    ) -> Result<()> {
        let config = self.config()?;
        let relative_path = self.relative_path();
        let branch_name = self.branch_name();
        let solutions_problem_path = solutions_repo.join(relative_path);
        let repository_problem_path = self.path.clone();
        if checkout_branch && !launch_git!(solutions_repo, "checkout", &branch_name) {
            bail!("failed to checkout branch in solutions repository")
        }
        copy_files(
            &solutions_problem_path,
            &repository_problem_path,
            config.get_relative_user_files(),
            false,
        )
    }

    pub fn move_solution_files_to(&self, solutions_repo: &Path) -> Result<()> {
        let config = self.config()?;
        let relative_path = self.relative_path();
        let branch_name = self.branch_name();
        let solutions_problem_path = solutions_repo.join(relative_path);
        let repository_problem_path = self.path.clone();
        if !launch_git!(solutions_repo, "checkout", &branch_name)
            && !launch_git!(
                solutions_repo,
                "checkout",
                "-b",
                &branch_name,
                "origin/master"
            )
            && !launch_git!(
                solutions_repo,
                "checkout",
                "-b",
                &branch_name,
                "origin/main"
            )
        {
            bail!("failed to both checkout and create branch in solutions repository")
        }
        copy_files(
            &repository_problem_path,
            &solutions_problem_path,
            config.get_relative_user_files(),
            true,
        )
    }
}
