use git2::Repository;
use crate::modules::rgb::rgb;

pub fn git_status() -> String {
    let repo = Repository::discover(".").ok();
    if repo.is_none() {
        return "".to_string();
    }
    let repo = repo.unwrap();

    let branch = repo.head()
        .ok()
        .and_then(|h| h.shorthand().map(|s| s.to_string()))
        .unwrap_or("DETACHED".to_string());

    let statuses = repo.statuses(None).unwrap();

    let mut state_colored = rgb(102, 255, 102, "Clean");

    let mut has_dirty = false;
    let mut has_staged = false;

    for entry in statuses.iter() {
        let s = entry.status();
        if s.is_wt_modified() || s.is_wt_new() {
            has_dirty = true;
            break;
        } else if s.is_index_modified() || s.is_index_new() || s.is_index_deleted() {
            has_staged = true;
        }
    }

    if has_dirty {
        state_colored = rgb(248, 81, 73, "Dirty");
    } else if has_staged {
        state_colored = rgb(227, 179, 65, "Staged");
    }

    let branch_colored = rgb(180, 135, 255, &branch);
    format!(" -  {} {}", branch_colored, state_colored)
}

