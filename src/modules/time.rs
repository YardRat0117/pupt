use crate::modules::rgb::rgb;
use chrono::Local;

pub fn current_time() -> String {
    let now = Local::now();
    let time_str = now.format("%a %m-%d %H:%M").to_string();
    rgb(227, 179, 65, &time_str).to_string()
}
