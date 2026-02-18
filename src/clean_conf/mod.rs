use std::path::Path;

use crate::err::Result;

mod exist_run;

pub use self::exist_run::*;

pub trait CleanConf {
    fn name(&self) -> &str;

    fn matches(&self, path: &Path) -> bool;

    fn clean(&mut self, path: &Path) -> Result<()>;
}
