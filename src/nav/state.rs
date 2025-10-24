use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct NavigationState {
    signal: Signal<Navigation>,
}

impl NavigationState {
    pub fn init(is_entries_empty: bool) -> Self {
        let value = if is_entries_empty {
            Navigation::Import
        } else {
            Navigation::Chart
        };
        let signal = use_signal(|| value);
        Self { signal }
    }

    pub fn get(&self) -> Navigation {
        *self.signal.read()
    }

    pub fn set(&mut self, value: Navigation) {
        self.signal.set(value);
    }

    pub fn is_active(&self, value: Navigation) -> bool {
        self.get() == value
    }
}
