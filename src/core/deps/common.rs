use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum DepSource {
    Path(String),
    Git {
        url: String,
        branch: Option<String>,
        tag: Option<String>,
        rev: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub struct DepSpec {
    pub name: String,
    pub source: DepSource,
    pub include_raw: Option<Vec<String>>,
    pub lib_raw: Option<Vec<String>>,
    pub libs_raw: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct ResolvedDeps {
    pub include_dirs: Vec<String>,
    pub lib_dirs: Vec<String>,
    pub libs: Vec<String>,
}

pub fn resolve_path(project_root: &Path, raw: &str) -> Result<PathBuf, String> {
    let p = Path::new(raw);
    let full = if p.is_absolute() {
        p.to_path_buf()
    } else {
        project_root.join(p)
    };
    Ok(full)
}

pub fn resolve_paths(
    base: &Path,
    raw: Option<&[String]>,
    defaults: &[&str],
    profile: &str,
) -> Result<Vec<PathBuf>, String> {
    let mut out = Vec::new();
    if let Some(raw) = raw {
        for r in raw {
            let expanded = expand_profile(r, profile);
            let p = Path::new(&expanded);
            let full = if p.is_absolute() {
                p.to_path_buf()
            } else {
                base.join(p)
            };
            if !full.exists() {
                return Err(format!("Path does not exist: {}", full.display()));
            }
            out.push(full);
        }
        return Ok(out);
    }

    for d in defaults {
        let candidate = base.join(d);
        if candidate.exists() {
            out.push(candidate);
        }
    }
    Ok(out)
}

pub fn expand_profile(raw: &str, profile: &str) -> String {
    raw.replace("{profile}", profile)
}

pub fn sync_dep_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    if dst.exists() {
        fs::remove_dir_all(dst)?;
    }
    copy_dir_all(src, dst)
}

pub fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}
