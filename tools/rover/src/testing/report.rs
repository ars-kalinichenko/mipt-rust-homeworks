use anyhow::{bail, Context, Result};
use reqwest::blocking::{multipart::Form, Client};
use std::{env, thread, time};

const MANYTASK_URL: &str = "https://mipt-rust.manytask.org/api/report";
const MANYTASK_RETRIES: usize = 3;

pub enum ReportType {
    NoReport,
    Manytask,
}

impl ReportType {
    pub fn from_name(name: &str) -> Result<Self> {
        Ok(match name {
            "no-report" => Self::NoReport,
            "manytask" => Self::Manytask,
            name => bail!("report type \"{name}\" is not supported"),
        })
    }

    pub fn push_report(&self, failed: bool) -> Result<()> {
        match self {
            Self::NoReport => {
                if failed {
                    bail!("testing failed")
                } else {
                    Ok(())
                }
            }
            Self::Manytask => {
                if env::var("SKIP_REPORT").is_ok() {
                    return Ok(());
                }
                let task_name = env::var("CI_COMMIT_REF_NAME")
                    .context("no CI_COMMIT_REF_NAME variable")?
                    .split('/')
                    .nth(1)
                    .context("CI_COMMIT_REF_NAME does not contain '/' symbol")?
                    .to_owned();
                let user_id = env::var("GITLAB_USER_ID").context("no GITLAB_USER_ID variable")?;
                let tester_token = env::var("TESTER_TOKEN").context("no TESTER_TOKEN variable")?;
                let client = Client::new();
                for _ in 0..MANYTASK_RETRIES {
                    let mut data = Form::new()
                        .text("user_id", user_id.clone())
                        .text("task", task_name.clone())
                        .text("token", tester_token.clone());
                    if failed {
                        data = data.text("failed", "1");
                    }
                    if client
                        .post(MANYTASK_URL)
                        .multipart(data)
                        .send()
                        .context("post report to manytask")?
                        .status()
                        != 500
                    {
                        return Ok(());
                    } else {
                        thread::sleep(time::Duration::from_millis(1000));
                    }
                }
                bail!("{MANYTASK_RETRIES} posts to manytask gave 500 code")
            }
        }
    }
}
