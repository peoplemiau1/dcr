use std::path::Path;

pub fn fetch_git_dep(
    url: &str,
    target_dir: &Path,
    branch: Option<&str>,
    tag: Option<&str>,
    rev: Option<&str>,
) -> Result<(), String> {
    if target_dir.exists() {
        return Ok(());
    }

    let mut builder = git2::build::RepoBuilder::new();
    if let Some(b) = branch {
        builder.branch(b);
    }

    let repo = builder
        .clone(url, target_dir)
        .map_err(|e| format!("failed to clone {}: {}", url, e))?;

    if let Some(t) = tag {
        checkout_ref(&repo, &format!("refs/tags/{}", t))?;
    } else if let Some(r) = rev {
        checkout_ref(&repo, r)?;
    }

    Ok(())
}

fn checkout_ref(repo: &git2::Repository, reference_str: &str) -> Result<(), String> {
    let (object, reference) = repo
        .revparse_ext(reference_str)
        .map_err(|e| format!("failed to find ref {}: {}", reference_str, e))?;

    repo.checkout_tree(&object, None).map_err(|e| {
        let ref_name = reference.as_ref().and_then(|r| r.name()).unwrap_or("HEAD");
        format!("failed to checkout {}: {}", ref_name, e)
    })?;

    match reference {
        Some(gref) => {
            let ref_name = gref.name().unwrap_or("HEAD");
            repo.set_head(ref_name)
        }
        None => repo.set_head_detached(object.id()),
    }
    .map_err(|e| format!("failed to set HEAD: {}", e))?;

    Ok(())
}
