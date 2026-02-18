pub fn help() {
    println!(
        "Welcome in help fro cleanr by BonnyAD9.

Usage:
  cleanr [flags] [directories]
    Recursively cleans build files from all projects in the directories.

Flags:
  -h  --help
    Show this help.

  -d  --directory <path>
    Explicitly specify directory to clean.

  -s  --skip <name>
    Skips all directories with the given name.

  --dry
    Dry run. Only show what will happen.
"
    )
}
