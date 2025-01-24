use crate::{
    macros::{KeyMacros, KeyMacrosBuilder, TriggerKey},
    term::{Color, println_with_color},
};
use dialoguer::{Select, theme::ColorfulTheme};
use rdev::{EventType, Key, listen, simulate};
use std::{thread::sleep, time::Duration};

const WELCOME_TEXT: &str = r"  __  __   ___   ____    _       _
 |  \/  | |_ _| / ___|  (_)   __| |   ___     _ __ ___     __ _    ___   _ __    ___    ___
 | |\/| |  | |  \___ \  | |  / _` |  / _ \   | '_ ` _ \   / _` |  / __| | '__|  / _ \  / __|
 | |  | |  | |   ___) | | | | (_| | |  __/   | | | | | | | (_| | | (__  | |    | (_) | \__ \
 |_|  |_| |___| |____/  |_|  \__,_|  \___|   |_| |_| |_|  \__,_|  \___| |_|     \___/  |___/
";

// const ALT: &str = r" ___      ___   __      ________  __     ________    _______      ___      ___       __       ______    _______     ______    ________
// |'  \    /'  | |' \    /'       )|' \   |'      '\  /'     '|    |'  \    /'  |     /''\     /' _  '\  /'      \   /    ' \  /'       )
//  \   \  //   | ||  |  (:   \___/ ||  |  (.  ___  :)(: ______)     \   \  //   |    /    \   (: ( \___)|:        | // ____  \(:   \___/
//  /\\  \/.    | |:  |   \___  \   |:  |  |: \   ) || \/    |       /\\  \/.    |   /' /\  \   \/ \     |_____/   )/  /    ) :)\___  \
// |: \.        | |.  |    __/  \\  |.  |  (| (___\ || // ___)_     |: \.        |  //  __'  \  //  \ _   //      /(: (____/ //  __/  \\
// |.  \    /:  | /\  |\  /' \   :) /\  |\ |:       :)(:      '|    |.  \    /:  | /   /  \\  \(:   _) \ |:  __   \ \        /  /' \   :)
// |___|\__/|___|(__\_|_)(_______/ (__\_|_)(________/  \_______)    |___|\__/|___|(___/    \___)\_______)|__|  \___) \'_____/  (_______/";

const SIMULATE_KEY: Key = Key::Escape;

fn print_trigger() {
    println_with_color(
        "Press desired trigger (except '\\' - backslash): ",
        Color::Magenta,
    );
}

pub fn run() {
    println_with_color(WELCOME_TEXT, Color::Magenta);
    let options = vec!["Start macros", "Exit"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(" Select an option")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    println!("");

    if selection == 1 {
        return;
    }

    let mut key_macros: Option<KeyMacros> = None;
    print_trigger();
    if let Err(err) = listen(move |event| match key_macros.clone() {
        Some(macros) => {
            if let EventType::KeyPress(key) = event.event_type {
                if key == Key::BackSlash {
                    key_macros = None;
                    print_trigger();
                }
            }

            if let Ok(action) = TriggerKey::try_from(event.event_type) {
                if macros.should_be_invoked(&action) {
                    println_with_color("Macros invoked", Color::Blue);
                    macros.invoke();
                }
            }
        }

        None => {
            let trigger_key = TriggerKey::try_from(event.event_type);
            match trigger_key {
                Ok(k) => {
                    println_with_color(
                        format!("Selected trigger: {}", event.name.unwrap_or_default()),
                        Color::Green,
                    );
                    key_macros = Some({
                        KeyMacrosBuilder::new(k)
                            .with_actions(vec![
                                || _ = simulate(&EventType::KeyPress(SIMULATE_KEY)),
                                || _ = simulate(&EventType::KeyRelease(SIMULATE_KEY)),
                                || sleep(Duration::from_millis(1000 / 60)),
                                || _ = simulate(&EventType::KeyPress(SIMULATE_KEY)),
                                || _ = simulate(&EventType::KeyRelease(SIMULATE_KEY)),
                            ])
                            .build()
                    });
                    println_with_color("Macros activated!", Color::Blue);
                }
                Err(_) => {}
            }
        }
    }) {
        println!("Error: {:?}", err);
    };
}
