use tokio::fs;
use crate::modules::rgb::rgb;

pub async fn battery_status() -> String {
    let base = "/sys/class/power_supply/BATT";

    let capacity_file = fs::read_to_string(format!("{}/capacity", base))
        .await
        .unwrap_or_default();
    let capacity = capacity_file.trim().to_string();

    let cap_value: i32 = capacity.parse().unwrap_or(0);
    let battery_str = format!("{}%", capacity);

    let battery_colored = match cap_value {
        80..=100 => rgb(86, 211, 100, &battery_str),
        30..=79 => rgb(227, 179, 65, &battery_str),
        _ => rgb(248, 81, 73, &battery_str),
    };

    battery_colored.to_string()
}

