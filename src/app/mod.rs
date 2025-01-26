mod print;
mod scenarios;

use dialoguer::{Select, theme::ColorfulTheme};

pub fn run() {
    print::welcome();

    let selection = select_scenario();
    print::newline();

    match selection {
        1 => scenarios::EscMacros::run(),
        _ => return,
    };
}

fn select_scenario() -> usize {
    let options = vec![
        "Exit",           // 0
        "Run ESC marcos", // 1
    ];

    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(" Select an option")
        .items(&options)
        .default(0)
        .interact()
        .unwrap()
}
