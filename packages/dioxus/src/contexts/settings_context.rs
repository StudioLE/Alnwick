use crate::prelude::*;

/// Global settings [context](https://dioxuslabs.com/learn/0.6/reference/context/).
#[derive(Clone, Copy, Debug)]
pub struct SettingsContext {
    pub skip_forward: Signal<Option<u32>>,
    pub skip_back: Signal<Option<u32>>,
}

impl SettingsContext {
    /// Creates a new instance of the context.
    #[must_use]
    pub fn new() -> Self {
        Self {
            skip_forward: use_signal(|| None),
            skip_back: use_signal(|| None),
        }
    }
}
