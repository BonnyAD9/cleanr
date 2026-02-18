use std::{
    fmt::Display,
    fs::{DirEntry, read_dir},
    path::PathBuf,
};

use crate::{clean_conf::CleanConf, err::Result};

pub fn direct(
    mut dirs: Vec<PathBuf>,
    confs: &mut [Box<dyn CleanConf>],
    skip: Vec<String>,
    dry: bool,
) {
    'outer: while let Some(d) = dirs.pop() {
        if !d.is_dir() {
            continue;
        }

        if let Some(n) = d.file_name()
            && skip.iter().any(|a| n == a.as_str())
        {
            eprintln!("Skipping `{}`", d.display());
            continue;
        }

        for c in &mut *confs {
            if c.matches(&d) {
                eprintln!("Running `{}` in `{}`", c.name(), d.display());
                if !dry {
                    let e = c.clean(&d);
                    log_err(
                        format!("Failed to clean with `{}` in `{}`", c.name(), d.display()),
                        e,
                    );
                }
                continue 'outer;
            }
        }

        let Some(r) = log_err(
            "Failed to recurse directory",
            read_dir(d).map_err(|e| e.into()),
        ) else {
            continue;
        };

        for d in r {
            log_err("Failed to add entry", try_add_entry(d, &mut dirs));
        }
    }
}

fn try_add_entry(
    d: std::result::Result<DirEntry, std::io::Error>,
    stack: &mut Vec<PathBuf>,
) -> Result<()> {
    let d = d?;
    if d.file_type()?.is_dir() {
        stack.push(d.path());
    }

    Ok(())
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
