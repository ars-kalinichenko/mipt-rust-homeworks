use crate::{launch_git, repository::repo::Repository};
use anyhow::{bail, Result};
use std::{
    path::{Path, PathBuf},
    process,
};

pub fn submit_problem(
    problem_path: &Path,
    message: &str,
    solutions_repo: Option<PathBuf>,
) -> Result<()> {
    let repository = Repository::from_path(problem_path)?;
    let problem = repository.problem_from_path(problem_path)?;
    let solutions_repo = match solutions_repo {
        Some(path) => path,
        None => repository.solutions_repo()?,
    };
    problem.move_solution_files_to(&solutions_repo)?;
    if !launch_git!(&solutions_repo, "add", ".") {
        bail!("git add failed");
    }
    if !launch_git!(&solutions_repo, "commit", "-m", message) {
        bail!("git commit failed: either no changes since the last commit or git failed")
    }
    if !launch_git!(&solutions_repo, "push")
        && !launch_git!(
            &solutions_repo,
            "push",
            "--set-upstream",
            "origin",
            problem.branch_name()
        )
    {
        bail!("git push failed")
    }
    Ok(())
}
