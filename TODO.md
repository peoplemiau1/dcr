# DCR TODO.md

Priority list of tasks to turn DCR into a true Cargo killer for C/C++ developers.

## 🔥 High Priority (Next Releases)

- [x] **Git dependencies support**  
  `git = "https://github.com/user/repo.git#branch"` and `git = "..."@tag`
- [ ] **Conan and vcpkg integration**  
  Automatic download, installation and linking of packages
- [ ] **Full freestanding / embedded support**  
  `-nostdlib`, linker scripts, bare-metal targets (i686-elf, aarch64-none-elf, etc.)
- [ ] **EFI support**  
  Dedicated platform configuration for EFI targets
- [ ] **Custom project name**  
  Override binary name in `dcr.toml`
- [ ] **Custom build steps / targets**  
  Ability to define custom pre-build, post-build, codegen steps in `dcr.toml`
- [ ] **Improved mixed C/C++/ASM projects**  
  Better handling of `.S` and `.asm` files with correct flags
- [ ] **dcr publish + official package registry**  
  Publish and fetch your own libraries easily

## ✅ Medium Priority

- [ ] **Static analysis** (`dcr check`)  
  Integration with clang-tidy, cppcheck, include-what-you-use
- [ ] **Linter + Formatter** (`dcr fmt`, `dcr lint`)
- [ ] **Hot reload / Live reload** for applications (raylib, SFML, SDL, etc.)
- [ ] **Better cross-compilation experience**  
  Ready-to-use toolchains + automatic download
- [ ] **Linker script support** + direct linker control
- [ ] **Workspaces improvements**  
  Per-member include directories, selective builds, better dependency graph
- [ ] **Build-time code generation**  
  Support for protobuf, flatbuffers, Qt moc, custom generators

## 📌 Low Priority / Nice to Have

- [ ] Support for Meson and Ninja as alternative backends
- [ ] Dockerfile generation (`dcr docker init`)
- [ ] First-class Windows MSVC support (currently works but rough)
- [ ] Documentation generation (`dcr doc`)
- [ ] Sanitizers integration (`dcr sanitize`)
- [ ] Benchmarks and comparison with CMake/Meson
- [ ] C++20 modules support (when it becomes stable)

## 🧠 Future Ideas

- Official DCR package registry (dcrhub)
- GUI wrapper (dcr-gui)
- Plugin system
- Toolchain manager (like rustup)
- Zig as alternative compiler
- deb/rpm/flatpak/AppImage packaging support

---

**Currently in progress:**
- ...

**Completed in recent releases:**
- Workspaces
- Cross-compilation
- IDE config generation (VS Code, CLion, compile_commands.json)
- Self-update (`dcr --update`)
- Header tracking
