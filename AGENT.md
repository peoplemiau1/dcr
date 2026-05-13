# AGENT.md

## Project Overview

**DCR (Dexoron Cargo Realization)** — CLI tool for managing C/C++/ASM projects in a Cargo-style.

- **Implementation Language**: Rust (edition 2024)
- **License**: MIT
- **Repository**: [github.com/dexoron/dcr](https://github.com/dexoron/dcr)

## Architecture

### Source Tree (`src/`)

```
src/
├── main.rs          # Entry point, CLI command routing
├── config.rs        # Global constants
├── cli/             # CLI command implementation
│   ├── new.rs       # dcr new <name>
│   ├── init.rs      # dcr init
│   ├── setup.rs     # dcr setup (registry initialization)
│   ├── add.rs       # dcr add <name>
│   ├── build.rs     # dcr build
│   ├── run.rs       # dcr run
│   ├── clean.rs     # dcr clean
│   ├── help.rs      # dcr --help
│   └── flag_update.rs  # dcr --update
├── core/            # Business logic
│   ├── config.rs    # dcr.toml parsing
│   ├── registry.rs  # Package index management (RegistryManager)
│   ├── deps/        # Dependency resolution (path, git)
│   ├── runner.rs    # Binary execution
│   ├── workspace.rs # [workspace] support
│   └── builder/     # Compiler backends (gcc, clang, msvc, nasm, gas)
├── platform/        # Platform-specific logic
└── utils/           # Helper utilities
```

### Key Concepts

- **`~/.dcr/`**: Centralized storage for registries and configurations.
- **`config.toml`**: File in `~/.dcr/` defining available package indices and their priorities.
- **`dcr.toml`**: Local project configuration.
- **Registry System**: `RegistryManager` handles package discovery across multiple indices based on priority.

## Supported Platforms

| Platform | Target triple |
|---|---|
| Linux | `x86_64-unknown-linux-gnu` |
| macOS Intel | `x86_64-apple-darwin` |
| macOS ARM | `aarch64-apple-darwin` |
| Windows | `x86_64-pc-windows-msvc` |

## Dependencies (Cargo)

| Crate | Purpose |
|---|---|
| `reqwest` | HTTP requests |
| `serde` | Serialization |
| `toml` | Configuration parsing |
| `git2` | Git dependency handling |

## Development

### Build & Run
```bash
cargo build --release
./target/release/dcr setup
```

## Code Conventions
- Rust stable edition 2024.
- `cargo fmt` and `cargo clippy -- -D warnings`.
- CLI output in English.
- Use `utils::log` for messaging.
