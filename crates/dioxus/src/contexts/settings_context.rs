use crate::prelude::*;

/// Global settings [context](https://dioxuslabs.com/learn/0.6/reference/context/).
#[derive(Clone, Copy, Debug)]
pub struct SettingsContext {
    pub skip_forward: Signal<Option<u32>>,
    pub skip_back: Signal<Option<u32>>,
}

impl SettingsContext {
    /// Creates a new instance of the context.
    ///
    /// This should be called at the top of the `App` component.
    pub fn create() {
        let context = Self {
            skip_forward: use_signal(|| None),
            skip_back: use_signal(|| None),
        };
        use_context_provider(|| context);
    }

    /// Consume the context from the current scope.
    #[must_use]
    pub fn consume() -> Self {
        consume_context()
    }
}
