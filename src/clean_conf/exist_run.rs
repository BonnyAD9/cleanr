use std::{borrow::Cow, ffi::OsStr, path::Path, process::Command};

use crate::{clean_conf::CleanConf, err::Result};

pub struct ExistRun {
    pub name: Cow<'static, str>,
    pub dirs: Vec<Cow<'static, Path>>,
    pub files: Vec<Cow<'static, Path>>,
    pub cmd: Command,
    pub cwd: Option<Cow<'static, Path>>,
}

impl ExistRun {
    pub fn new<S: AsRef<OsStr>, I: IntoIterator<Item = S>>(
        name: impl Into<Cow<'static, str>>,
        dirs: Vec<Cow<'static, Path>>,
        files: Vec<Cow<'static, Path>>,
        cmd: impl AsRef<OsStr>,
        args: I,
        cwd: Option<Cow<'static, Path>>,
    ) -> Self {
        let mut cmd = Command::new(cmd);
        cmd.args(args);
        Self {
            name: name.into(),
            dirs,
            files,
            cmd,
            cwd,
        }
    }
}

impl CleanConf for ExistRun {
    fn name(&self) -> &str {
        &self.name
    }

    fn matches(&self, path: &Path) -> bool {
        self.dirs
            .iter()
            .map(|a| path.join(a).is_dir())
            .chain(self.files.iter().map(|a| path.join(a).is_file()))
            .all(|a| a)
    }

    fn clean(&mut self, path: &Path) -> Result<()> {
        if let Some(cwd) = self.cwd.as_deref() {
            self.cmd.current_dir(cwd);
        } else {
            self.cmd.current_dir(path);
        }

        self.cmd.spawn()?.wait()?;
        Ok(())
    }
}
