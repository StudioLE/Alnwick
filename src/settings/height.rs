use crate::prelude::*;

const NBSP: char = '\u{00A0}';

#[derive(Copy, Clone)]
struct FieldState {
    /// Global height context
    state: HeightState,
    /// Current field value
    value: Signal<String>,
    /// Validation messages
    message: Signal<Option<String>>,
}

impl FieldState {
    pub fn init() -> Self {
        let state: HeightState = use_context();
        let value = state.get_cm_string();
        Self {
            state,
            value: use_signal(|| value),
            message: use_signal(|| None),
        }
    }

    fn is_valid(&self) -> bool {
        self.message.read().is_none()
    }

    fn get_class(&self) -> String {
        if self.is_valid() {
            "input".to_owned()
        } else {
            "input is-danger".to_owned()
        }
    }

    fn get_message(&self) -> String {
        self.message.read().clone().unwrap_or_default()
    }

    fn oninput(&mut self, event: Event<FormData>) {
        event.prevent_default();
        let input_value = event.value();
        self.value.set(input_value.clone());
        match parse_input(input_value) {
            Ok(height) => {
                self.state.set(Some(height));
                self.message.set(None);
            }
            Err(message) => {
                self.state.set(None);
                self.message.set(Some(message));
            }
        }
    }
}

#[component]
pub(crate) fn Height() -> Element {
    let mut state = FieldState::init();
    rsx! {
        div { class: "field",
            label { class: "label", "Height" }
            div { class: "field has-addons",
                p { class: "control",
                    input {
                        oninput: move |event| state.oninput(event),
                        class: state.get_class(),
                        r#type: "text",
                        placeholder: "Example: 162",
                        value: state.value,
                    }
                }
                p { class: "control",
                    a { class: "button is-static", "cm" }
                }
            }
            if state.is_valid() {
                p { class: "help", dangerous_inner_html: "{NBSP}" }
            } else {
                p { class: "help is-danger", "{state.get_message()}" }
            }
        }
    }
}

fn parse_input(input: String) -> Result<f32, String> {
    let Ok(cm) = input.parse::<f32>() else {
        return Err("Height must be a number".to_owned());
    };
    if !(50.0..=300.0).contains(&cm) {
        return Err("Height must be between 50 and 300 cm".to_owned());
    }
    Ok(cm / 100.0)
}
