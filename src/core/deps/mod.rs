pub mod common;
pub mod git;
pub mod lock;
pub mod register;

use crate::core::config::Config;
use crate::core::deps::common::ResolvedDeps;
use std::path::Path;

pub fn resolve_deps(
    config: &Config,
    _profile: &str,
    _target: Option<&str>,
    project_root: &Path,
) -> Result<ResolvedDeps, String> {
    let mut resolved = ResolvedDeps::default();
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

    let deps_table = config.get("dependencies").and_then(|v| v.as_table());
    let lock_packages = Vec::new();

    if let Some(deps) = deps_table {
        for (name, value) in deps {
            if register::is_registry_dep(value) {
                let _pkg_info = register::resolve_package_from_registry(name)?;
                // Используем путь относительно project_root или системный путь, если нужно
                let dep_root = project_root.join("dcr-index");

                resolved.include_dirs.push(
                    dep_root
                        .join("target")
                        .join("include")
                        .to_string_lossy()
                        .to_string(),
                );
                resolved.lib_dirs.push(
                    dep_root
                        .join("target")
                        .join("lib")
                        .to_string_lossy()
                        .to_string(),
                );
                resolved.libs.push(name.clone());
            } else if let Some(table) = value.as_table()
                && let Some(path) = table.get("path").and_then(|v| v.as_str())
            {
                let dep_root = project_root.join(path);
                if let Some(includes) = table.get("include").and_then(|v| v.as_array()) {
                    for inc in includes {
                        if let Some(inc_str) = inc.as_str() {
                            resolved
                                .include_dirs
                                .push(dep_root.join(inc_str).to_string_lossy().to_string());
                        }
                    }
                }
                let lib_path = dep_root.join("target").join("lib");
                if lib_path.exists() {
                    resolved
                        .lib_dirs
                        .push(lib_path.to_string_lossy().to_string());
                }
                if let Some(libs) = table.get("libs").and_then(|v| v.as_array()) {
                    for lib in libs {
                        if let Some(lib_str) = lib.as_str() {
                            resolved.libs.push(lib_str.to_string());
                        }
                    }
                } else {
                    resolved.libs.push(name.clone());
                }
            }
        }
    }

    crate::core::deps::lock::write_lock(
        project_root,
        &project_name,
        &project_version,
        &lock_packages,
    )?;

    Ok(resolved)
}
