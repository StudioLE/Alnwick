use crate::pages::settings::player::state::FieldState;
use crate::prelude::*;

const NBSP: char = '\u{00A0}';

#[component]
pub(crate) fn FieldComponent() -> Element {
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
                        value: state.get_value(),
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
