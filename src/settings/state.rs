use crate::prelude::*;

const KEY: &str = "height";

#[derive(Clone, Copy, Debug)]
pub struct HeightState {
    /// Height in meters
    ///
    /// `None` if not set or invalid
    signal: Signal<Option<f32>>,
}

impl HeightState {
    pub fn init() -> Self {
        let value = get_from_local_storage();
        let signal = use_signal(|| value);
        Self { signal }
    }

    pub fn get(&self) -> Option<f32> {
        *self.signal.read()
    }

    pub fn get_cm_string(&self) -> String {
        let Some(height) = self.get() else {
            return String::new();
        };
        (height * 100.0).to_string()
    }

    pub fn set(mut self, value: Option<f32>) {
        self.signal.set(value);
        if let Some(value) = value {
            let _ = set_to_local_storage(value);
        }
    }
}

fn get_from_local_storage() -> Option<f32> {
    LocalStorage::get(KEY).handle_error(|e| warn!("Failed to get {KEY} from local storage: {e:?}"))
}

fn set_to_local_storage(value: f32) -> bool {
    LocalStorage::set(KEY, &value)
        .handle_error(|e| warn!("Failed to set {KEY} in local storage: {e:?}"))
        .is_some()
}
