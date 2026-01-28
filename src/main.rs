mod modules;
use modules::{battery, dir, git, time};

#[tokio::main]
async fn main() {
    let git = git::git_status().await;
    let battery = battery::battery_status().await;
    let dir = dir::short_dir().await;
    let time = time::current_time().await;

    let prompt = format!("╭─ [{} {}] {}{}\n╰─❯ ", battery, time, dir, git);

    println!("{}", prompt);
}
