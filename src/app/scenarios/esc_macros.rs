use std::cell::RefCell;
use std::rc::Rc;

use rdev::{Event, EventType, Key, listen};

use crate::app::print;
use crate::macros::{KeyMacros, KeyMacrosBuilder, PrepandAction, TriggerKey};

#[derive(Default)]
enum Action {
    #[default]
    BindTrigger,
    RunningMacros(KeyMacros),
}

#[derive(Default)]
pub struct EscMacros {
    current_action: Action,
    next_action: Option<Action>,
}

impl EscMacros {
    pub fn new() -> Self {
        Self {
            current_action: Action::default(),
            next_action: Some(Action::BindTrigger), // We should bind trigger at first.
        }
    }

    pub fn run(mut self) {
        if let Err(err) = listen(move |event| self.listen(event)) {
            print::errorln(format!("Error: {err:?}"));
        }
    }

    fn listen(&mut self, event: Event) {
        self.handle_next_action();
        self.handle_event(event);
    }

    fn handle_event(&mut self, event: Event) {
        let trigger_key = TriggerKey::try_from(event.event_type).ok();

        match &self.current_action {
            Action::RunningMacros(macros) => {
                if let Some(key) = trigger_key {
                    // Should rebind?
                    if let Some(rebind_action) = self.handle_rebind_keys(&key) {
                        self.next_action.replace(rebind_action);
                        // self.next_action = Some(rebind_action);
                        return;
                    }
                    // Should invoke macros?
                    if macros.should_be_invoked(&key) {
                        print::successln("Macros invoked");
                        macros.invoke();
                    }
                }
            },

            Action::BindTrigger => match trigger_key {
                // Valid key -> rebuild macros with new trigger
                Some(key) => {
                    print::clear_current_line();
                    print::infoln(format!("Selected trigger: {key:?}"));
                    print::infoln("To rebind, press `\\`");

                    self.next_action.replace(Action::RunningMacros(
                        KeyMacrosBuilder::new(key).build_prepand(PrepandAction::EscMacros),
                    ));
                },
                // Invalid key -> Still have to Bind Trigger -> current_action should stay unchanged
                None => {},
            },
        }
    }

    fn handle_rebind_keys(&self, key: &TriggerKey) -> Option<Action> {
        match key {
            TriggerKey::Keyboard(Key::BackSlash) => Some(Action::BindTrigger),
            _ => None,
        }
    }

    /// Inform user what to do next and update ***current action***.
    ///
    /// ```
    /// self.current_action = self.next_action.take();
    /// ```
    fn handle_next_action(&mut self) {
        let Some(next_action) = self.next_action.take() else {
            return;
        };

        match &next_action {
            Action::BindTrigger => {
                print::info("Press desired trigger (except '\\'):");
            },
            Action::RunningMacros(macros) => {
                print::successln(format!("Macros build: {macros}"));
            },
        };

        self.current_action = next_action;
    }
}
