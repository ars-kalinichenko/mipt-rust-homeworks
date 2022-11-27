use anyhow::{bail, Context, Result};
use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

pub fn copy_files(
    src: &Path,
    dest: &Path,
    relative_paths: &[PathBuf],
    create_new: bool,
) -> Result<()> {
    for file in relative_paths {
        if !src.join(file).is_file() {
            bail!("there's no needed file in solutions repository")
        }
    }
    for file in relative_paths {
        let src = src.join(file);
        let dest = dest.join(file);
        let mut src_file = BufReader::new(File::open(&src).context("unable to open file")?);
        if create_new {
            create_dir_all(dest.parent().context("destination file has no parent")?)
                .context("error while creating needed folders")?;
        }
        let mut dst_file = BufWriter::new(
            OpenOptions::new()
                .create_new(!dest.exists())
                .write(true)
                .truncate(true)
                .open(&dest)
                .with_context(|| format!("cannot open destination file {:?}", dest))?,
        );
        let mut buf = Vec::new();
        src_file
            .read_to_end(&mut buf)
            .context("io error while reading")?;
        dst_file
            .write_all(buf.as_slice())
            .context("io error while writing")?;
    }
    Ok(())
}
