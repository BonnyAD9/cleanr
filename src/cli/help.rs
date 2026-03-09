use termal::{gradient, printacln};

pub fn help() {
    let v = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
    let signature = gradient("BonnyAD9", (250, 50, 170), (180, 50, 240));

    printacln!(
        "Welcome in help fro {'g i}cleanr{'_} by {signature}{'_}.
Version {v}

Cleanr is small utility that automatizes cleaning build files across all your
projects.

{'g}Usage:
  {'c}cleanr {'gr}[{'dy}flags{'gr}] [directories]{'_}
    Recursively cleans build files from all projects in the directories.

{'g}Flags:
  {'y}-h  --help{'_}
    Show this help.

  {'y}-d  --directory {'w}<path>{'_}
    Explicitly specify directory to clean.

  {'y}-s  --skip {'w}<name>
    Skips all directories with the given name.

  {'y}--dry{'_}
    Dry run. Only show what will happen.

 “ {'i}Be strong, all you people of the land,’ declares the Lord,
   ‘and work. For I am with you,’ declares the Lord Almighty. {'_}”
                                                   {'w bold}✝ Haggai 2:4{'_}
"
    )
}
