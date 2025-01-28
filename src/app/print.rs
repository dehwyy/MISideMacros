use std::fmt::Display;

use crate::term::{Color, print_colored};

const WELCOME_TEXT: &str = r"  __  __   ___   ____    _       _
 |  \/  | |_ _| / ___|  (_)   __| |   ___     _ __ ___     __ _    ___   _ __    ___    ___
 | |\/| |  | |  \___ \  | |  / _` |  / _ \   | '_ ` _ \   / _` |  / __| | '__|  / _ \  / __|
 | |  | |  | |   ___) | | | | (_| | |  __/   | | | | | | | (_| | | (__  | |    | (_) | \__ \
 |_|  |_| |___| |____/  |_|  \__,_|  \___|   |_| |_| |_|  \__,_|  \___| |_|     \___/  |___/
";

// pub fn line_up() {
//     print!("\r\x1b[1A");
// }

pub fn clear_current_line() {
    print!("\r\x1b[0J");
}

pub fn newline() {
    println!();
}

pub fn welcome() {
    print_colored(format!("{WELCOME_TEXT}\n\n"), Color::Yellow);
}

// pub fn guideln(s: impl Display) {
//     guide(format!("{s}\n"));
// }

// pub fn guide(s: impl Display) {
//     print_colored(s, Color::Cyan);
// }

pub fn infoln(s: impl Display) {
    info(format!("{s}\n"));
}

pub fn info(s: impl Display) {
    print_colored(s, Color::Blue);
}

pub fn successln(s: impl Display) {
    success(format!("{s}\n"));
}

pub fn success(s: impl Display) {
    print_colored(s, Color::Green);
}

pub fn errorln(s: impl Display) {
    print_colored(format!("{s}\n"), Color::Red);
}
