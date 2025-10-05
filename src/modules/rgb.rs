pub fn rgb(r: u8, g: u8, b: u8, s: &str) -> String {
    format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, s)
}
