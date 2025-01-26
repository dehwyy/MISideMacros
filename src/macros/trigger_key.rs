use rdev::{Button, EventType, Key};

#[derive(Clone, PartialEq, Eq, Debug)]
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
