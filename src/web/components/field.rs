mod field_state;

use crate::prelude::*;
use field_state::*;

const NBSP: char = '\u{00A0}';

#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Props, Clone, PartialEq)]
pub struct FieldProps<T>
where
    T: 'static + Clone + Copy + PartialEq,
{
    /// Label displayed above the field
    label: String,
    /// Unit displayed at the end of the field
    unit: String,
    /// Placeholder text when the field is empty
    placeholder: String,
    /// Global value
    global_value: Signal<Option<T>>,
    /// Convert from input text to a value value to a
    from_string: fn(String) -> Result<T, String>,
    /// Convert from a value to input text
    to_string: fn(Option<T>) -> String,
}

#[component]
pub(crate) fn Field<T>(props: FieldProps<T>) -> Element
where
    T: 'static + Clone + Copy + PartialEq,
{
    let mut state = FieldState::new(props.clone());
    rsx! {
        div { class: "field",
            label { class: "label", "{props.label}" }
            div { class: "field has-addons",
                p { class: "control",
                    input {
                        oninput: move |event| state.oninput(event),
                        class: state.get_class(),
                        r#type: "text",
                        placeholder: props.placeholder,
                        value: state.get_value(),
                    }
                }
                p { class: "control",
                    a { class: "button is-static", "{props.unit}" }
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
