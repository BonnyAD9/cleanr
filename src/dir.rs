use std::{fs::read_dir, path::PathBuf};

use crate::{err::Result, file::File, item::Item};

#[derive(Debug)]
pub struct Dir {
    path: PathBuf,
    size: u64,
    contents: Vec<Item>,
}

impl Dir {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let mut size = 0;
        let mut contents = vec![];

        for i in read_dir(&path)? {
            let i = i?;
            let t = i.file_type()?;
            if t.is_file() {
                let f = File::new(i.path())?;
                size += f.size();
                contents.push(Item::File(f));
            } else if t.is_dir() {
                let d = Dir::new(i.path())?;
                size += d.size();
                contents.push(Item::Dir(d));
            }
        }

        Ok(Self {
            path,
            size,
            contents,
        })
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}
