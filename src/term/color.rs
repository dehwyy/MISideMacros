#[allow(dead_code)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Black,
    White,
}

impl Color {
    pub(super) fn ansi_code(&self) -> u8 {
        match self {
            Color::Red => 31,
            Color::Green => 32,
            Color::Blue => 34,
            Color::Yellow => 33,
            Color::Cyan => 36,
            Color::Magenta => 35,
            Color::Black => 30,
            Color::White => 37,
        }
    }
}
