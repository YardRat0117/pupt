use git2::Repository;
use crate::modules::rgb::rgb;

pub fn git_status() -> String {
    let repo = Repository::discover(".").ok();
    let repo = match repo {
        Some(r) => r,
        None => return "".to_string(),
    };

    let branch = repo
        .head()
        .ok()
        .and_then(|h| h.shorthand().map(|s| s.to_string()))
        .unwrap_or("DETACHED".to_string());

    let statuses = repo.statuses(None).unwrap();
    let state_colored = if !statuses.is_empty() {
        rgb(248, 81, 73, "D")
    } else {
        rgb(86, 211, 100, "C")
    };

    let branch_colored = rgb(201, 209, 217, &branch);

    format!(" {} {}", branch_colored, state_colored)
}

