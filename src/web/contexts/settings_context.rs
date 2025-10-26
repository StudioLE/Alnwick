use crate::prelude::*;

/// Global settings [context](https://dioxuslabs.com/learn/0.6/reference/context/).
#[derive(Clone, Copy, Debug)]
pub struct SettingsContext {
    signal: Signal<Option<f32>>,
}

impl SettingsContext {
    /// Creates a new instance of the context.
    pub fn init() -> Self {
        let signal = use_signal(|| None);
        Self { signal }
    }

    /// Get the current settings.
    pub fn get(&self) -> Option<f32> {
        *self.signal.read()
    }

    /// Set the value.
    pub fn set(&mut self, value: Option<f32>) {
        self.signal.set(value);
        // TODO MUST Set value on server
    }
}
