use crate::prelude::*;

/// [Context](https://dioxuslabs.com/learn/0.6/reference/context/) of the current page.
#[derive(Clone, Copy, Debug)]
pub struct PageContext {
    signal: Signal<PageInfo>,
}

impl PageContext {
    /// Creates a new instance of the context.
    pub fn init() -> Self {
        let signal = use_signal(|| PageInfo::default());
        Self { signal }
    }

    /// Get the current page.
    pub fn get(&self) -> PageInfo {
        self.signal.read().clone()
    }

    /// Set the current page.
    pub fn set(&mut self, value: PageInfo) {
        self.signal.set(value);
    }

    /// Check if the current page is the given value.
    pub fn is_active(&self, value: PageInfo) -> bool {
        self.get() == value
    }
}
