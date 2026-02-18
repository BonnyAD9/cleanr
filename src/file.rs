use std::path::PathBuf;

use crate::err::Result;

#[derive(Debug)]
pub struct File {
    path: PathBuf,
    size: u64,
}

impl File {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let size = path.metadata()?.len();
        Ok(Self { path, size })
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}
