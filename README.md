# cleanr

Bulk cleans build files from your projects.

## Supported project types
- If `Makefile` exists, runs `make clean`.
- If `CMakeLists.txt` and `build` exists, runs `make clean` in `build`.
- If `Cargo.toml` exists, runs `cargo clean`.
- If `gleam.toml` exists, runs `gleam clean`.
- If `.zig-cache` exists, runs `rm -rf .zig-cache`.
- If `*.csproj` exists, runs `dotnet clean`.