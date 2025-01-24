use rdev::{Button, EventType, Key};

type Action = fn() -> ();

#[derive(Clone, PartialEq, Eq)]
pub enum TriggerKey {
    Keyboard(Key),
    MouseButton(Button),
}

impl TryFrom<EventType> for TriggerKey {
    type Error = ();
    fn try_from(event_type: EventType) -> Result<Self, Self::Error> {
        match event_type {
            EventType::KeyPress(key) => Ok(TriggerKey::Keyboard(key)),
            EventType::ButtonPress(button) => Ok(TriggerKey::MouseButton(button)),
            _ => Err(()),
        }
    }
}

pub struct KeyMacrosBuilder {
    trigger_key: TriggerKey,
    actions: Vec<Action>,
}

impl KeyMacrosBuilder {
    pub fn new(trigger_key: TriggerKey) -> KeyMacrosBuilder {
        KeyMacrosBuilder {
            trigger_key,
            actions: Vec::new(),
        }
    }

    pub fn with_actions(mut self, actions: Vec<Action>) -> Self {
        self.actions.extend(actions);
        self
    }

    pub fn build(self) -> KeyMacros {
        KeyMacros {
            trigger_key: self.trigger_key,
            actions: self.actions,
        }
    }
}
#[derive(Clone)]
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
