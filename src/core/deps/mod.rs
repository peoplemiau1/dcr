pub mod common;
pub mod git;
pub mod lock;

use crate::core::config::Config;
use std::fs;
use std::path::Path;
use toml::Value;

pub use common::{DepSource, DepSpec, ResolvedDeps};
use common::{expand_profile, resolve_path, resolve_paths, sync_dep_dir};
use git::fetch_git_dep;
use lock::{DepLock, compute_checksum, read_dep_version, write_lock};

pub fn resolve_deps(
    config: &Config,
    profile: &str,
    target: Option<&str>,
    project_root: &Path,
) -> Result<ResolvedDeps, String> {
    let project_name = config
        .get("package.name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let project_version = config
        .get("package.version")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let deps = parse_dependencies(config, profile, target)?;
    if deps.is_empty() {
        return Ok(ResolvedDeps {
            include_dirs: Vec::new(),
            lib_dirs: Vec::new(),
            libs: Vec::new(),
        });
    }

    let mut include_dirs = Vec::new();
    let mut lib_dirs = Vec::new();
    let mut libs = Vec::new();
    let mut lock_packages = Vec::new();

    for dep in deps {
        let (dep_path, is_git) = match &dep.source {
            DepSource::Path(path_raw) => {
                let expanded = expand_profile(path_raw, profile);
                (resolve_path(project_root, &expanded)?, false)
            }
            DepSource::Git {
                url,
                branch,
                tag,
                rev,
            } => {
                let git_deps_dir = project_root
                    .join("target")
                    .join(profile)
                    .join("deps")
                    .join("git");
                fs::create_dir_all(&git_deps_dir)
                    .map_err(|e| format!("failed to create target/{}/deps/git: {}", profile, e))?;
                let dep_dir = git_deps_dir.join(&dep.name);
                if !dep_dir.exists() {
                    use crate::utils::text::{BOLD_GREEN, colored};
                    use std::io::{self, Write};
                    print!(
                        "    {} dependency `{}` from {}...",
                        colored("Fetching", BOLD_GREEN),
                        dep.name,
                        url
                    );
                    io::stdout().flush().ok();

                    fetch_git_dep(
                        url,
                        &dep_dir,
                        branch.as_deref(),
                        tag.as_deref(),
                        rev.as_deref(),
                    )?;

                    print!(
                        "\r\x1B[2K    {} dependency `{}`\n",
                        colored("Fetched", BOLD_GREEN),
                        dep.name
                    );
                    io::stdout().flush().ok();
                } else {
                    fetch_git_dep(
                        url,
                        &dep_dir,
                        branch.as_deref(),
                        tag.as_deref(),
                        rev.as_deref(),
                    )?;
                }
                (dep_dir, true)
            }
        };

        if !dep_path.is_dir() {
            return Err(format!(
                "Dependency '{}' path is not a directory: {}",
                dep.name,
                dep_path.display()
            ));
        }

        let include = resolve_paths(&dep_path, dep.include_raw.as_deref(), &["include"], profile)?;
        let lib = resolve_paths(
            &dep_path,
            dep.lib_raw.as_deref(),
            &["lib", "lib64"],
            profile,
        )?;
        let libs_list = dep
            .libs_raw
            .clone()
            .unwrap_or_else(|| vec![dep.name.clone()]);

        if include.is_empty() || lib.is_empty() {
            return Err(format!(
                "Dependency '{}' missing include/lib dirs. Add include/lib in dcr.toml.",
                dep.name
            ));
        }

        include_dirs.extend(include.iter().map(|p| p.to_string_lossy().to_string()));
        lib_dirs.extend(lib.iter().map(|p| p.to_string_lossy().to_string()));
        libs.extend(libs_list.iter().cloned());

        if !is_git {
            let dep_cache = project_root
                .join("target")
                .join(profile)
                .join("deps")
                .join(&dep.name);
            sync_dep_dir(&dep_path, &dep_cache)
                .map_err(|err| format!("Failed to sync dep {}: {err}", dep.name))?;
        }

        let dep_version = read_dep_version(&dep_path).unwrap_or_else(|| "0.0.0".to_string());
        let dep_checksum = compute_checksum(&dep_path)
            .map_err(|err| format!("Failed to hash dep {}: {err}", dep.name))?;
        let source_str = match &dep.source {
            DepSource::Path(p) => format!("path+{}", p),
            DepSource::Git { url, .. } => format!("git+{}", url),
        };
        lock_packages.push(DepLock {
            name: dep.name.clone(),
            version: dep_version,
            checksum: dep_checksum,
            source: source_str,
        });
    }

    write_lock(
        project_root,
        &project_name,
        &project_version,
        &lock_packages,
    )?;

    Ok(ResolvedDeps {
        include_dirs,
        lib_dirs,
        libs,
    })
}

fn parse_dependencies(
    config: &Config,
    profile: &str,
    target: Option<&str>,
) -> Result<Vec<DepSpec>, String> {
    let mut deps_table = None;
    let combinations = if let Some(t) = target {
        let normalized_t = crate::cli::build::normalize_target_os(t);
        vec![
            format!("dependencies.{}.{}", normalized_t, profile),
            format!("dependencies.{}.{}", profile, normalized_t),
            format!("dependencies.{}", normalized_t),
            format!("dependencies.{}", profile),
            "dependencies".to_string(),
        ]
    } else {
        vec![
            format!("dependencies.{}", profile),
            "dependencies".to_string(),
        ]
    };
    for key in combinations {
        if let Some(val) = config.get(&key).and_then(|v| v.as_table()) {
            deps_table = Some(val);
            break;
        }
    }
    let deps_table = match deps_table {
        Some(t) => t,
        None => return Ok(Vec::new()),
    };
    let mut deps = Vec::new();
    for (name, value) in deps_table {
        match value {
            Value::String(path) => {
                deps.push(DepSpec {
                    name: name.to_string(),
                    source: DepSource::Path(path.to_string()),
                    include_raw: None,
                    lib_raw: None,
                    libs_raw: None,
                });
            }
            Value::Table(tbl) => {
                if tbl.get("system").and_then(|v| v.as_bool()).unwrap_or(false) {
                    return Err(format!(
                        "Dependency '{}' uses system=true, which is not supported yet",
                        name
                    ));
                }

                let source = if let Some(git_url) = tbl.get("git").and_then(|v| v.as_str()) {
                    DepSource::Git {
                        url: git_url.to_string(),
                        branch: tbl
                            .get("branch")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        tag: tbl
                            .get("tag")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        rev: tbl
                            .get("rev")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                    }
                } else if let Some(path) = tbl.get("path").and_then(|v| v.as_str()) {
                    DepSource::Path(path.to_string())
                } else {
                    return Err(format!("Dependency '{}' missing path or git", name));
                };

                let include = parse_string_list(tbl.get("include"), name, "include")?;
                let lib = parse_string_list(tbl.get("lib"), name, "lib")?;
                let libs = parse_string_list(tbl.get("libs"), name, "libs")?;
                deps.push(DepSpec {
                    name: name.to_string(),
                    source,
                    include_raw: include,
                    lib_raw: lib,
                    libs_raw: libs,
                });
            }
            _ => {
                return Err(format!("Dependency '{}' must be a string or a table", name));
            }
        }
    }
    deps.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(deps)
}

fn parse_string_list(
    value: Option<&Value>,
    dep_name: &str,
    key: &str,
) -> Result<Option<Vec<String>>, String> {
    let Some(value) = value else { return Ok(None) };
    let list = value.as_array().ok_or_else(|| {
        format!(
            "Dependency '{}' field '{}' must be an array of strings",
            dep_name, key
        )
    })?;
    let mut out = Vec::new();
    for item in list {
        let s = item
            .as_str()
            .ok_or_else(|| format!("Dependency '{}' field '{}' must be strings", dep_name, key))?;
        out.push(s.to_string());
    }
    Ok(Some(out))
}
