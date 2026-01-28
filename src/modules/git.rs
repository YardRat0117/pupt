use crate::modules::rgb::rgb;
use git2::{Repository, Status, StatusOptions};
use tokio::task;

fn discover_repo() -> Option<Repository> {
    let mut current = std::env::current_dir().ok()?;
    loop {
        if let Ok(repo) = Repository::open(&current) {
            return Some(repo);
        }
        match current.parent() {
            Some(p) => current = p.to_path_buf(),
            None => break,
        }
    }
    None
}

pub async fn git_status() -> String {
    task::spawn_blocking(|| {
        let repo = match discover_repo() {
            Some(r) => r,
            None => return "".to_string(),
        };

        let branch = repo
            .head()
            .ok()
            .and_then(|h| h.shorthand().map(|s| s.to_string()))
            .unwrap_or_else(|| "DETACHED".to_string());

        // 检查 staged 文件
        let mut has_staged = false;
        if let Ok(index) = repo.index() {
            for entry in index.iter() {
                let stage = (entry.flags >> 12) & 0b11; // stage 位
                if stage != 0 {
                    has_staged = true;
                    break;
                }
            }
        }

        // 检查 untracked 文件
        let mut has_dirty = false;
        let mut opts = StatusOptions::new();
        opts.include_untracked(true)
            .include_ignored(false)
            .include_unmodified(false)
            .recurse_untracked_dirs(true)
            .renames_head_to_index(true)
            .renames_index_to_workdir(true);

        if let Ok(statuses) = repo.statuses(Some(&mut opts)) {
            has_dirty = statuses.iter().any(|s| {
                s.status().intersects(
                    Status::WT_NEW | Status::WT_MODIFIED | Status::WT_DELETED | Status::WT_RENAMED,
                )
            });
        }

        let state_colored = if has_dirty {
            rgb(248, 81, 73, "Dirty")
        } else if has_staged {
            rgb(227, 179, 65, "Staged")
        } else {
            rgb(102, 255, 102, "Clean")
        };

        let branch_colored = rgb(180, 135, 255, &branch);
        format!(" -  {} {}", branch_colored, state_colored)
    })
    .await
    .unwrap_or_else(|_| "".to_string())
}
