mod modules;
use modules::{battery, dir, git, time};

#[tokio::main]
async fn main() {
    let battery = battery::battery_status().await;
    let dir = dir::short_dir();
    let git = git::git_status();
    let time = time::current_time();

    let prompt = format!("╭─ [{} {}] {} - {}\n╰─❯ ", battery, time, dir, git);

    println!("{}", prompt);
}
