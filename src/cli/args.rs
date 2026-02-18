use pareg::Pareg;

use crate::err::Result;

use super::help::help;

#[derive(Debug, Default)]
pub struct Args {
    pub dirs: Vec<String>,
}

impl Args {
    pub fn parse(mut args: Pareg) -> Result<Self> {
        let mut res = Self::default();
        let mut helped = false;

        while let Some(arg) = args.next() {
            match arg {
                "-h" | "-?" | "--help" => {
                    help();
                    helped = true;
                }
                "-d" | "--directory" => res.dirs.push(args.next_arg()?),
                v if v.starts_with("-") => {
                    let msg = format!("If you want to clean directory `{v}` use `-d`");
                    return args.err_unknown_argument().hint(msg).err()?;
                }
                _ => res.dirs.push(args.cur_arg()?),
            }
        }

        if res.dirs.is_empty() && !helped {
            res.dirs.push(".".to_string());
        }

        Ok(res)
    }
}
