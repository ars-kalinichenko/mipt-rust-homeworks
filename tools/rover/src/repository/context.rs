use std::path::{Path, PathBuf};

pub struct CommandContext {
    workdir: PathBuf,
    user_files: Vec<PathBuf>,
}

impl CommandContext {
    pub fn new(workdir: &Path, user_files: &[PathBuf]) -> Self {
        Self {
            workdir: workdir.to_path_buf(),
            user_files: user_files.to_vec(),
        }
    }

    pub fn get_workdir(&self) -> &Path {
        &self.workdir
    }

    pub fn get_user_files(&self) -> &[PathBuf] {
        &self.user_files
    }
}
