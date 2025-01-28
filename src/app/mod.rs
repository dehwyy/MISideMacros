mod print;
mod scenarios;

use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

pub fn run() {
    print::welcome();

    let selection = select_scenario();
    print::newline();

    if selection == 1 {
        scenarios::EscMacros::new().run();
    }
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
