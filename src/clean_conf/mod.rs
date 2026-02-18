use std::path::Path;

use crate::err::Result;

mod dotnet;
mod exist_run;

pub use self::{dotnet::*, exist_run::*};

pub trait CleanConf {
    fn name(&self) -> &str;

    fn matches(&self, path: &Path) -> bool;

    fn clean(&mut self, path: &Path) -> Result<()>;
}
