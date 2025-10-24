use crate::prelude::*;

/// Local state of the input field.
#[derive(Copy, Clone)]
pub(super) struct FieldState {
    /// Global settings context
    context: SettingsContext,
    /// Current field value
    value: Signal<String>,
    /// Validation messages
    message: Signal<Option<String>>,
}

impl FieldState {
    pub(super) fn init() -> Self {
        let context: SettingsContext = use_context();
        let value = context.get();
        let value = to_string(value);
        Self {
            context,
            value: use_signal(|| value),
            message: use_signal(|| None),
        }
    }

    pub(super) fn is_valid(&self) -> bool {
        self.message.read().is_none()
    }

    pub(super) fn get_class(&self) -> String {
        if self.is_valid() {
            "input".to_owned()
        } else {
            "input is-danger".to_owned()
        }
    }

    pub(super) fn get_value(&self) -> String {
        self.value.read().clone()
    }

    pub(super) fn get_message(&self) -> String {
        self.message.read().clone().unwrap_or_default()
    }

    pub(super) fn oninput(&mut self, event: Event<FormData>) {
        event.prevent_default();
        let input_value = event.value();
        self.value.set(input_value.clone());
        match from_string(input_value) {
            Ok(height) => {
                self.context.set(Some(height));
                self.message.set(None);
            }
            Err(message) => {
                self.context.set(None);
                self.message.set(Some(message));
            }
        }
    }
}

fn from_string(input: String) -> Result<f32, String> {
    let Ok(cm) = input.parse::<f32>() else {
        return Err("Height must be a number".to_owned());
    };
    if !(50.0..=300.0).contains(&cm) {
        return Err("Height must be between 50 and 300 cm".to_owned());
    }
    Ok(cm / 100.0)
}

fn to_string(value: Option<f32>) -> String {
    let Some(height) = value else {
        return String::new();
    };
    (height * 100.0).to_string()
}
