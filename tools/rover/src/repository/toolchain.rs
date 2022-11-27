use super::{command::Command, context::CommandContext};
use anyhow::{bail, Context, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    process,
};

const FORBID_UNSAFE_LINE: &str = "#![forbid(unsafe_code)]";
const FORBID_STD_LINE: &str = "#![no_std]";
const FORBID_COLLECTIONS_PATTERNS: [&str; 8] = [
    "BTreeMap",
    "BTreeSet",
    "HashMap",
    "HashSet",
    "Vec",
    "VecDeque",
    "LinkedList",
    "BinaryHeap",
];

macro_rules! launch {
    ($toolchain: expr, $command: expr, $context: expr) => {{
        let toolchain_shell_line = $toolchain.get_shell_line()?;
        let command_shell_line = $command.get_shell_line()?;
        let toolchain_shell_iter = toolchain_shell_line.split(' ');
        let command_shell_iter = command_shell_line.split(' ');
        let mut iter = toolchain_shell_iter.chain(command_shell_iter);
        let mut cmd = if let Some(program) = iter.next() {
            let mut cmd = process::Command::new(program);
            cmd.current_dir($context.get_workdir());
            cmd
        } else {
            bail!("toolchain and command are empty")
        };
        while let Some(arg) = iter.next() {
            cmd.arg(arg);
        }
        if cmd.status().context("command failed")?.success() {
            Ok(())
        } else {
            bail!("command failed")
        }
    }};
}

#[derive(Clone, Copy, Debug)]
pub enum Toolchain {
    Empty,
    Stable,
    Nightly,
}

impl Toolchain {
    pub fn from_name(name: &str) -> Result<Self> {
        Ok(match name {
            "empty" => Self::Empty,
            "stable" => Self::Stable,
            "nightly" => Self::Nightly,
            name => bail!("toolchain \"{name}\" is not supported"),
        })
    }

    pub fn get_shell_line(&self) -> Result<String> {
        Ok(match self {
            Self::Empty => "".to_string(),
            Self::Stable => "rustup run stable".to_string(),
            Self::Nightly => "rustup run nightly".to_string(),
        })
    }

    pub fn run_command(&self, command: &Command, context: &CommandContext) -> Result<()> {
        match command {
            Command::ForbidUnsafe => {
                for file in context.get_user_files() {
                    if let Some(line) = BufReader::new(File::open(file)?)
                        .lines()
                        .next()
                        .transpose()?
                    {
                        if line != FORBID_UNSAFE_LINE {
                            bail!(format!(
                                "file {file:?} does not contain line '{FORBID_UNSAFE_LINE}'"
                            ))
                        }
                    } else {
                        // TODO: ForbidUnsafe shouldn't check whether file is empty
                        bail!(format!("file {file:?} is empty"))
                    }
                }
                Ok(())
            }
            Command::ForbidCollections => {
                for file in context.get_user_files() {
                    for line in BufReader::new(File::open(file)?).lines() {
                        let line = line.context("error reading file line")?;
                        for pattern in FORBID_COLLECTIONS_PATTERNS {
                            if line.contains(pattern) {
                                bail!(format!("file {file:?} contains line '{pattern}'"))
                            }
                        }
                    }
                }
                Ok(())
            }
            Command::ForbidStd => {
                for file in context.get_user_files() {
                    if let Some(line) = BufReader::new(File::open(file)?)
                        .lines()
                        .next()
                        .transpose()?
                    {
                        if line != FORBID_STD_LINE {
                            bail!(format!(
                                "file {file:?} does not contain line '{FORBID_STD_LINE}'"
                            ))
                        }
                    } else {
                        // TODO: ForbidStd shouldn't check whether file is empty
                        bail!(format!("file {file:?} is empty"))
                    }
                }
                Ok(())
            }
            Command::CargoCompileTestMiniFrunk => {
                // TODO: hardcoded, better to refactor all the code
                if process::Command::new("cargo")
                    .current_dir(context.get_workdir())
                    .arg("test")
                    .arg("--features")
                    .arg("compilation-fail-generic")
                    .status()
                    .context("command failed")?
                    .success()
                {
                    bail!("command failed")
                }
                if process::Command::new("cargo")
                    .current_dir(context.get_workdir())
                    .arg("test")
                    .arg("--features")
                    .arg("compilation-fail-labelled")
                    .status()
                    .context("command failed")?
                    .success()
                {
                    bail!("command failed")
                }
                if process::Command::new("cargo")
                    .current_dir(context.get_workdir())
                    .arg("test")
                    .arg("--features")
                    .arg("compilation-fail-transmogrify")
                    .status()
                    .context("command failed")?
                    .success()
                {
                    bail!("command failed")
                }
                println!("Compile tests passed, don't worry :)");
                Ok(())
            }
            Command::CargoCompileTestOrm => {
                // TODO: hardcoded, better to refactor all the code
                if process::Command::new("cargo")
                    .current_dir(context.get_workdir())
                    .arg("test")
                    .arg("--features")
                    .arg("test-lifetimes-create")
                    .status()
                    .context("command failed")?
                    .success()
                {
                    bail!("command failed")
                }
                if process::Command::new("cargo")
                    .current_dir(context.get_workdir())
                    .arg("test")
                    .arg("--features")
                    .arg("test-lifetimes-get")
                    .status()
                    .context("command failed")?
                    .success()
                {
                    bail!("command failed")
                }
                println!("Compile tests passed, don't worry :)");
                Ok(())
            }
            Command::CargoCompileTestSnapshot => {
                // TODO: hardcoded, better to refactor all the code
                if process::Command::new("cargo")
                    .current_dir(context.get_workdir())
                    .arg("test")
                    .arg("--features")
                    .arg("test-lifetime")
                    .status()
                    .context("command failed")?
                    .success()
                {
                    bail!("command failed")
                }
                println!("Compile tests passed, don't worry :)");
                Ok(())
            }
            Command::CargoFmt
            | Command::CargoClippy
            | Command::CargoTest
            | Command::CargoTestDebug
            | Command::PythonTest
            | Command::CargoMiriTest => {
                launch!(self, command, context)
            }
        }
    }
}
