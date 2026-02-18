use std::{path::Path, process::ExitCode};

use pareg::Pareg;

use crate::{clean_conf::CleanConf, cli::Args, err::Result};

mod clean;
mod clean_conf;
mod cli;
mod err;

fn main() -> ExitCode {
    match start() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn start() -> Result<()> {
    let args = Args::parse(Pareg::args())?;
    clean::direct(args.dirs, &mut default_configs(), args.skip, args.dry);
    Ok(())
}

fn default_configs() -> Vec<Box<dyn CleanConf>> {
    vec![
        Box::new(clean_conf::ExistRun::new(
            "make clean",
            vec![],
            vec![Path::new("Makefile").into()],
            "make",
            ["clean"],
            None,
        )),
        Box::new(clean_conf::ExistRun::new(
            "cmake with build",
            vec![Path::new("build").into()],
            vec![Path::new("CMakeLists.txt").into()],
            "make",
            ["clean"],
            Some(Path::new("build").into()),
        )),
        Box::new(clean_conf::ExistRun::new(
            "cargo clean",
            vec![],
            vec![Path::new("Cargo.toml").into()],
            "cargo",
            ["clean"],
            None,
        )),
        Box::new(clean_conf::ExistRun::new(
            "gleam clean",
            vec![],
            vec![Path::new("gleam.toml").into()],
            "gleam",
            ["clean"],
            None,
        )),
        Box::new(clean_conf::ExistRun::new(
            "zig remove .zig-cache",
            vec![Path::new(".zig-cache").into()],
            vec![],
            "rm",
            ["-rf", ".zig-cache"],
            None,
        )),
        Box::new(clean_conf::Dotnet),
    ]
}
