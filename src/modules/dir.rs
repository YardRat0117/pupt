use crate::modules::rgb::rgb;
use std::env;

pub fn short_dir() -> String {
    let path = env::current_dir().unwrap();
    let mut path_str = path.to_str().unwrap().to_string();
    let home = env::var("HOME").unwrap();
    path_str = path_str.replace(&home, "~");

    let parts: Vec<&str> = path_str.split('/').collect();
    let short = if parts.len() >= 2 {
        format!("{}/{}", parts[parts.len() - 2], parts[parts.len() - 1])
    } else {
        path_str
    };

    rgb(102, 198, 255, &short).to_string()
}
