use std::{thread::sleep, time::Duration};

use rdev::{EventType, Key, simulate};

use crate::boxed;

pub(super) type Action = Box<dyn Fn() -> ()>;

pub enum PrepandAction {
    EscMacros(u64), // (fps)
}

const SIMULATE_KEY: Key = Key::Escape;
impl PrepandAction {
    pub fn into_actions(self) -> Vec<Action> {
        match self {
            Self::EscMacros(fps) => vec![
                boxed!(|| _ = simulate(&EventType::KeyPress(SIMULATE_KEY))),
                boxed!(|| _ = simulate(&EventType::KeyRelease(SIMULATE_KEY))),
                boxed!(move || sleep(Duration::from_millis(1000 / fps))),
                boxed!(|| _ = simulate(&EventType::KeyPress(SIMULATE_KEY))),
                boxed!(|| _ = simulate(&EventType::KeyRelease(SIMULATE_KEY))),
            ],
        }
    }
}
