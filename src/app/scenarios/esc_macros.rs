use rdev::{Event, EventType, Key, listen};

use crate::{
    app::print,
    macros::{KeyMacros, KeyMacrosBuilder, PrepandAction, TriggerKey},
};

#[derive(Default)]
pub struct EscMacros {
    macros: Option<KeyMacros>,
    trigger: Option<TriggerKey>,
    fps: Option<u64>,
    raw_fps_input: Option<String>,
}

impl EscMacros {
    pub fn run() {
        let mut s = Self::default();
        if let Err(err) = listen(move |event| (&mut s).handle(event)) {
            eprintln!("Error: {:?}", err);
        }
    }

    fn handle(&mut self, event: Event) {
        // ? KeyRelease is used for print & rebinds. (action agnostic events)
        if let EventType::KeyRelease(_) = event.event_type {
            // * Should bind fps
            if self.fps.is_none() && self.raw_fps_input.is_none() {
                self.raw_fps_input = Some(String::new());
                print::infoln(
                    "To clarify, how FPS may affect macros, check out my Github repo: https://github.com/dehwyy/MISideSpeedrunMacros.",
                );
                print::guide("Enter FPS: ");
                return;
            }

            // * Should bind trigger
            if self.trigger.is_none() && self.raw_fps_input.is_none() {
                print::guideln("Press desired trigger (except '\\' - backslash):");
                return;
            }

            return;
        }

        // ? Check rebinds
        if let EventType::KeyPress(key) = event.event_type {
            match key {
                Key::BackSlash => {
                    // If NOT during FPS binding
                    if self.fps.is_some() {
                        // print::infoln("Trigger rebinding!");
                        self.trigger = None;
                        self.macros = None;
                        return;
                    }
                }
                Key::RightBracket => {
                    // If NOT during Trigger binding
                    if self.trigger.is_some() {
                        // print::infoln("FPS Rebinding!");
                        self.fps = None;
                        return;
                    }
                }
                _ => {}
            }
        }

        if let Some(macros) = &self.macros {
            if let Ok(key) = TriggerKey::try_from(event.event_type) {
                if macros.should_be_invoked(&key) && self.fps.is_some() {
                    macros.invoke();
                    print::successln("Macros invoked");
                    return;
                }
            }
        }

        // * Bind fps
        if let Some(fps) = &mut self.raw_fps_input {
            if let EventType::KeyPress(key) = event.event_type {
                match key {
                    Key::Backspace => {
                        if fps.len() > 0 {
                            _ = fps.pop();
                            print::info("\x1b[1D\x1b[0J"); // go 1 column left and erase to the end
                        }
                    }
                    Key::Return => {
                        let fps = fps.parse::<u64>().expect("Wrong fps handling! @dehwyy");
                        self.fps = Some(fps);
                        self.raw_fps_input = None;

                        print::clear_line();
                        print::successln(format!("FPS bound: {}", fps));
                        print::infoln("To rebind FPS, press `]`\n");

                        // ! Rebuild macros!
                        if let Some(trigger) = self.trigger.clone() {
                            self.macros = Some(build_macros(trigger, fps));
                        }
                    }
                    // Otherwise, try to parse input value
                    _ => {
                        if let Some(digit) = digit_from_key(key) {
                            print::info(format!("{digit}"));
                            fps.push_str(&digit.to_string());
                        }
                    }
                }
            }
            return;
        }

        // * Bind trigger
        if self.trigger.is_none() {
            if let Ok(trigger) = TriggerKey::try_from(event.event_type) {
                if matches!(
                    trigger,
                    TriggerKey::Keyboard(Key::BackSlash) | TriggerKey::Keyboard(Key::RightBracket)
                ) {
                    print::line_up();
                    print::clear_line();
                    return;
                }

                self.trigger = Some(trigger.clone());
                print::successln(format!(
                    "Selected trigger! <{}>",
                    event
                        .name
                        .unwrap_or("Could not display, but it works".to_string())
                ));
                print::infoln("To rebind trigger, press `\\`\n");

                // ! Rebuild macros!
                self.macros = Some(build_macros(trigger, self.fps.unwrap()));
            }
            return;
        }
    }
}

fn build_macros(trigger: TriggerKey, fps: u64) -> KeyMacros {
    print::successln(format!(
        "Macros built with: fps={fps}, triggerKey={trigger:?}",
    ));
    KeyMacrosBuilder::new(trigger).build_prepand(PrepandAction::EscMacros(fps))
}

fn digit_from_key(key: Key) -> Option<u64> {
    let digit: i32 = match key {
        Key::Num0 => 0,
        Key::Num1 => 1,
        Key::Num2 => 2,
        Key::Num3 => 3,
        Key::Num4 => 4,
        Key::Num5 => 5,
        Key::Num6 => 6,
        Key::Num7 => 7,
        Key::Num8 => 8,
        Key::Num9 => 9,
        _ => -1,
    };

    match digit {
        0..=9 => Some(digit as u64),
        _ => None,
    }
}
