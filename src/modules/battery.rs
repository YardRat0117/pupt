use crate::modules::rgb::rgb;
use tokio::fs;

pub async fn battery_status() -> String {
    let base = "/sys/class/power_supply/BATT";

    let capacity_file = fs::read_to_string(format!("{}/capacity", base))
        .await
        .unwrap_or_default();
    let capacity = capacity_file.trim().to_string();
    let cap_value: i32 = capacity.parse().unwrap_or(0);

    let voltage_file = fs::read_to_string(format!("{}/voltage_now", base))
        .await
        .unwrap_or_default();
    let voltage: f64 = voltage_file.trim().parse::<f64>().unwrap_or(0.0) / 1_000_000.0;

    let current_file = fs::read_to_string(format!("{}/current_now", base))
        .await
        .unwrap_or_default();
    let current: f64 = current_file.trim().parse::<f64>().unwrap_or(0.0) / 1_000_000.0;

    let power_watts = voltage * current;

    let battery_str = format!("{}%% ({:.2}W)", capacity, power_watts);

    let battery_colored = match cap_value {
        80..=100 => rgb(86, 211, 100, &battery_str),
        30..=79 => rgb(227, 179, 65, &battery_str),
        _ => rgb(248, 81, 73, &battery_str),
    };

    battery_colored.to_string()
}
