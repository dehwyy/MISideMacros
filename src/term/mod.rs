mod color;
use std::fmt::Display;
use std::io::{Write, stdout};

pub use color::Color;

pub fn print_colored(s: impl Display, color: Color) {
    print!("{}{}", ansi(color.ansi_code()), s);
    stdout().flush().expect("Failed to flush stdout")
}

fn ansi<S: Display>(s: S) -> String {
    format!("\x1b[{}m", s)
}
