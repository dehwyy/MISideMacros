use std::thread::sleep;
use std::time::Duration;

use rdev::{EventType, Key, simulate};

use crate::boxed;

const FPS: u64 = 60; // MISide game engine FPS (Unity)

pub(super) type Action = Box<dyn Fn()>;

pub enum PrepandAction {
    EscMacros,
}

const SIMULATE_KEY: Key = Key::Escape;
impl PrepandAction {
    pub fn into_actions(self) -> Vec<Action> {
        match self {
            Self::EscMacros => vec![
                boxed!(|| _ = simulate(&EventType::KeyPress(SIMULATE_KEY))),
                boxed!(|| _ = simulate(&EventType::KeyRelease(SIMULATE_KEY))),
                boxed!(move || sleep(Duration::from_millis(1000 / FPS))),
                boxed!(|| _ = simulate(&EventType::KeyPress(SIMULATE_KEY))),
                boxed!(|| _ = simulate(&EventType::KeyRelease(SIMULATE_KEY))),
            ],
        }
    }
}
