use crate::repository::problem::Problem;
use anyhow::Result;

pub fn test_problem(problem: Problem) -> Result<()> {
    problem.launch_all_steps()
}
