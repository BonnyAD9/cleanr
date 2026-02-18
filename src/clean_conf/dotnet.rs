use std::{
    ffi::OsStr,
    fmt::Display,
    fs::{DirEntry, read_dir},
    path::Path,
    process::Command,
};

use crate::{clean_conf::CleanConf, err::Result};

pub struct Dotnet;

impl CleanConf for Dotnet {
    fn name(&self) -> &str {
        "dotnet clean"
    }

    fn matches(&self, path: &Path) -> bool {
        log_err("Failed to match dotnet", try_match(path)).unwrap_or_default()
    }

    fn clean(&mut self, path: &Path) -> Result<()> {
        Command::new("dotnet")
            .arg("clean")
            .current_dir(path)
            .spawn()?
            .wait()?;
        Ok(())
    }
}

fn try_match(d: &Path) -> Result<bool> {
    Ok(read_dir(d)?
        .any(|d| log_err("dotnet failed to try entry", try_entry(d)).unwrap_or_default()))
}

fn try_entry(e: std::result::Result<DirEntry, std::io::Error>) -> Result<bool> {
    let e = e?;
    Ok(e.file_type()?.is_file() && e.path().extension() == Some(OsStr::new("csproj")))
}

fn log_err<T>(msg: impl Display, e: Result<T>) -> Option<T> {
    match e {
        Err(e) => {
            eprintln!("{}: {}", msg, e);
            None
        }
        Ok(r) => Some(r),
    }
}
