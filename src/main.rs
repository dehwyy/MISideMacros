use std::{thread::sleep, time::Duration};

use rdev::{EventType, Key, listen, simulate};

const SIMULATE_KEY: Key = Key::Escape;

fn main() {
    if let Err(err) = listen(|event| match event.event_type {
        EventType::KeyPress(key) if key == Key::Num1 => {
            simulate(&EventType::KeyPress(SIMULATE_KEY));
            simulate(&EventType::KeyRelease(SIMULATE_KEY));
            sleep(Duration::from_millis(1000 / 60));
            simulate(&EventType::KeyPress(SIMULATE_KEY));
            simulate(&EventType::KeyRelease(SIMULATE_KEY));
        }
        _ => {}
    }) {
        println!("Error: {:?}", err);
    };
}
