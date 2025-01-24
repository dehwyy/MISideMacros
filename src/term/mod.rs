mod color;
use std::fmt::Display;

pub use color::Color;

pub fn println_with_color(s: impl Display, color: Color) {
    println!("{}{}", ansi(color.ansi_code()), s);
}

fn ansi<S: Display>(s: S) -> String {
    format!("\x1b[{}m", s)
}
