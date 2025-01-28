mod builder;
pub use builder::KeyMacrosBuilder;

mod trigger_key;
pub use trigger_key::TriggerKey;

mod action;
use action::Action;
pub use action::PrepandAction;

pub struct KeyMacros {
    trigger_key: TriggerKey,
    actions: Vec<Action>,
}

impl KeyMacros {
    pub fn should_be_invoked(&self, key: &TriggerKey) -> bool {
        self.trigger_key == *key
    }

    pub fn invoke(&self) {
        for action in &self.actions {
            action();
        }
    }
}

impl std::fmt::Display for KeyMacros {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyMacros")
            .field("trigger_key", &self.trigger_key)
            // .field("actions", &self.actions)
            .finish()
    }
}
