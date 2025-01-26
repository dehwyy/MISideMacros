use super::{Action, KeyMacros, PrepandAction, TriggerKey};

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

    // pub fn add_action(&mut self, action: Action) {
    //     self.actions.push(action);
    // }

    // pub fn with_action(mut self, action: Action) -> Self {
    //     self.actions.push(action);
    //     self
    // }

    // pub fn add_actions(&mut self, actions: Vec<Action>) {
    //     self.actions.extend(actions);
    // }

    // pub fn with_actions(mut self, actions: Vec<Action>) -> Self {
    //     self.actions.extend(actions);
    //     self
    // }

    pub fn build_prepand(mut self, prepand_action: PrepandAction) -> KeyMacros {
        self.actions.extend(prepand_action.into_actions());

        self.build()
    }

    pub fn build(self) -> KeyMacros {
        KeyMacros {
            trigger_key: self.trigger_key,
            actions: self.actions,
        }
    }
}
