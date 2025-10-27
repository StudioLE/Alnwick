use crate::prelude::*;

/// Local state of the input field.
#[derive(Copy, Clone)]
pub(super) struct FieldState {
    /// Global value
    global_value: Signal<Option<f32>>,
    /// Convert from input text to a value value to a
    from_string: fn(String) -> Result<f32, String>,
    /// Convert from a value to input text
    to_string: fn(Option<f32>) -> String,
    /// Current field value
    field_value: Signal<String>,
    /// Validation messages
    message: Signal<Option<String>>,
}

impl FieldState {
    pub(super) fn new(props: FieldProps) -> Self {
        let value = (props.to_string)(*props.global_value.read());
        Self {
            global_value: props.global_value,
            from_string: props.from_string,
            to_string: props.to_string,
            field_value: use_signal(|| value),
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
        self.field_value.read().clone()
    }

    pub(super) fn get_message(&self) -> String {
        self.message.read().clone().unwrap_or_default()
    }

    pub(super) fn oninput(&mut self, event: Event<FormData>) {
        event.prevent_default();
        let input_value = event.value();
        self.field_value.set(input_value.clone());
        match (self.from_string)(input_value) {
            Ok(height) => {
                self.global_value.set(Some(height));
                self.message.set(None);
            }
            Err(message) => {
                self.global_value.set(None);
                self.message.set(Some(message));
            }
        }
    }
}
