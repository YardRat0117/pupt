use colored::*;

pub fn rgb(r: u8, g: u8, b: u8, text: &str) -> ColoredString {
    text.truecolor(r, g, b)
}

