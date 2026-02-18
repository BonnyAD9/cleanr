use crate::{dir::Dir, file::File};

#[derive(Debug)]
pub enum Item {
    Dir(Dir),
    File(File),
}

impl Item {
    pub fn size(&self) -> u64 {
        match self {
            Self::Dir(d) => d.size(),
            Self::File(f) => f.size(),
        }
    }
}
