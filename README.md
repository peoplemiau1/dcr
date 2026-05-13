# DCR (Dexoron Cargo Realization)

DCR is a utility for managing C/C++ projects in a Cargo-like style.

The current implementation is written in Rust.

## Why DCR
- Unified project structure without manual setup
- Simple commands for common tasks
- Transparent compilation and predictable build profiles

## Features
- Create a new project or initialize the current directory
- Build a project with `debug` and `release` profiles
- Run the compiled binary
- Clean build artifacts
- Generate IDE integration files (VS Code, CLion) and compilation databases
- Generate a minimal C project template
- Build static and shared libraries
- Dependency management:
  - Git and Path dependencies with `dcr.lock`
  - Registry-based dependency resolution
- Library packaging functionality (auto-generate `include/` and `lib/` for `type = "lib"` packages)
- ASM projects with NASM/GAS or via GCC/Clang
- Mixed language projects
- Cross-compilation with `--target` (full triple support and short names)
- Target-specific configurations and inheritance
- Update the binary via `dcr --update` (GitHub Releases)

## Supported Platforms
- Linux: x86_64/aarch64 (GNU and Musl)
- macOS: x86_64/aarch64
- Windows: x86_64/aarch64 (MSVC, GNU, LLVM)

## Installation

### Package Manager

**Arch Linux**
```sh
yay -S dcr # or paru and other AUR package managers
```

### From Source

```sh
git clone https://github.com/dexoron/dcr.git
cd dcr
cargo build --release
mkdir -p ~/.local/bin
ln -sf "$PWD/target/release/dcr" ~/.local/bin/dcr
```

### Via `install.sh` (Linux/macOS)

```sh
curl -fsSL https://dcr.dexoron.su/install.sh | bash
```

### Via `install.ps1` (Windows)

```powershell
irm https://dcr.dexoron.su/install.ps1 | iex
```

When executed, both scripts ask whether to:
- download a prebuilt binary from GitHub Releases
- or build the project from `git`

Release assets:
- `dcr-x86_64-unknown-linux-gnu`
- `dcr-x86_64-apple-darwin`
- `dcr-aarch64-apple-darwin`
- `dcr-x86_64-pc-windows-msvc.exe`

## Update

- If DCR was installed from GitHub release assets, `install.sh`, `install.ps1`, or built manually:
  - use `dcr --update`
- If DCR is installed via `pacman/AUR`:
  - update with your package manager: `paru/yay -Syu dcr` or `sudo pacman -Syu dcr`
  - `dcr --update` detects package-managed installs and asks you to update via package manager

## Quick Start

Create a new project:
`dcr new hello`

Or initialize the current directory (the directory must be empty):
`dcr init`

Project structure:

```txt
hello/
- src/
- - main.c
- dcr.toml
```

Build and run the project:
`dcr run` or `dcr run --release`

## Commands

### `dcr new <name>`
Creates a project with the specified name in the current directory.

### `dcr init`
Creates a project in the current directory. The project name is taken from the directory name. The directory must be empty.

### `dcr setup`
Initializes the registry system by reading `~/.dcr/config.toml` and verifying the availability of configured indices.

### `dcr add <name> <source>`
Adds a dependency to `dcr.toml`.
If a registry is configured, `dcr` automatically finds the package.
Git/Path syntax: `path:`, `github:`, `gitlab:`, `git:`, or a full URL.
Use `--branch`, `--tag`, or `--rev` for Git dependencies.

### `dcr build [profile]`
Builds the project. If no profile is specified, `--debug` is used.
Use `--force` to rebuild without cache, `--clean` to clean before build.

### `dcr run [profile]`
Builds the project and runs the binary. If no profile is specified, `--debug` is used.
Use `--force` to rebuild without cache, `--clean` to clean before build.

Run manually:
`./target/<profile>/<name>`

### `dcr clean`
Removes the `target` directory in the project root.
Use `dcr clean --all` in a workspace root to clean all member projects.

### `dcr gen`
Generates IDE integration files and build tools.
- `dcr gen vscode`: VS Code workspace config
- `dcr gen clion`: CLion project files
- `dcr gen compile-commands`: `compile_commands.json` for clang tools

### `dcr test`
Runs the test suite.
Use `dcr test --init` to create test files.

## Build Profiles
Two profiles are supported:
- `--debug` (default) - built-in flags per compiler
- `--release` - built-in flags per compiler

Custom flags can be added in `dcr.toml` via `build.cflags` and `build.ldflags`.
Profile-specific flags can be added via `[build.debug]` and `[build.release]`.
You can also set `build.target` to override the output directory (profile-independent).
You can set `build.platform` to pass `-march=<platform>` (GCC/Clang) or `/arch:*` (MSVC) where supported.
Use `build.kind = "staticlib"` to build a static library instead of a binary.
Use `build.kind = "sharedlib"` to build a shared library (`.so`/`.dylib`/`.dll`).
`dcr run` is only for `build.kind = "bin"` and will fail for libraries.
Use `build.exclude`/`build.include` to control source/header collection.
Use `build.roots` and `build.src_disable` to replace the default `src/` root.
Use `build.steps` and `build.post_steps` to run custom commands.
Use `build.clean` for extra cleanup paths and `[run].cmd` to override run command.

## Cross-compilation

DCR supports cross-compilation via `--target <triple>` or short names:

```sh
dcr build --target linux --release    # x86_64-unknown-linux-gnu
dcr build --target macos --release    # x86_64-apple-darwin
dcr build --target windows --release  # x86_64-pc-windows-msvc
dcr run --target aarch64-linux-gnu
```

Configure multiple targets and target-specific settings in `dcr.toml`:

```toml
[build.targets]
targets = ["linux", "windows"]

[build.linux]
compiler = "gcc"
cflags = ["-O2"]

[build.windows]
compiler = "x86_64-w64-mingw32-gcc"
```

See [Cross-compilation guide](docs/dependencies-and-build/cross-compilation.md) for details.

## Configuration
The main project file is `dcr.toml`.
For multi-registry management, use `~/.dcr/config.toml`.

Example `dcr.toml`:

```toml
[package]
name = "hello"
version = "0.1.0"
type = "app" # or "lib", "none"

[build]
language = "c"
standard = "c11"
compiler = "clang"
kind = "bin" # "bin", "staticlib", "sharedlib"
# Optional platform hint
# platform = "x86_64"
# Optional custom flags
cflags = ["-Wall", "-Wextra"]
ldflags = ["-lm"]
#
# Optional include/exclude:
# exclude = ["src/vendor", "src/legacy/**"]
# include = ["src/boot/arch/**"]

[toolchain]
# Optional tool overrides:
# cc = "clang"
# cxx = "clang++"
# as = "as"
# ar = "ar"
# ld = "ld"
# uic = "uic"
# moc = "moc"
# rcc = "rcc"

[dependencies]
my_lib = "path:./libs/my_lib"
cool_lib = { git = "https://github.com/user/cool_lib", tag = "v1.0" }
```

Workspace example:
...
[workspace]
kernel = { path = "src/kernel", deps = ["core", "userspace"] }
core = { path = "src/core", deps = ["userspace"] }
userspace = { path = "src/userspace" }
```

Git and Path dependencies are supported. DCR will resolve them on build and generate `dcr.lock`.

Incremental build note: object files are rebuilt when source `.c/.cpp` or included header files change.


## Requirements
- Rust toolchain (`rustc`, `cargo`) - for building DCR from source
- C compiler (`clang`, `gcc`, or 'cl'(msvc))

## Releases
Releases are built automatically via GitHub Actions (`.github/workflows/release.yml`) when a tag matching `v*` is pushed.

## License
See `LICENSE`.
