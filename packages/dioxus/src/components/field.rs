use crate::prelude::*;
use std::ops::Deref;

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
pub fn Field<T>(mut props: FieldProps<T>) -> Element
where
    T: 'static + Clone + Copy + PartialEq,
{
    let initial_value = (props.to_string)(*props.global_value.read());
    let mut field_value = use_signal(|| initial_value);
    let mut message = use_signal(|| None);
    rsx! {
        div { class: "field",
            label { class: "label", "{props.label}" }
            div { class: "field has-addons",
                p { class: "control",
                    input {
                        oninput: move |event| {
                            event.prevent_default();
                            let input_value = event.value();
                            field_value.set(input_value.clone());
                            match (props.from_string)(input_value) {
                                Ok(height) => {
                                    props.global_value.set(Some(height));
                                    message.set(None);
                                }
                                Err(e) => {
                                    props.global_value.set(None);
                                    message.set(Some(e));
                                }
                            }
                        },
                        class: get_class(message),
                        r#type: "text",
                        placeholder: props.placeholder,
                        value: field_value,
                    }
                }
                p { class: "control",
                    a { class: "button is-static", "{props.unit}" }
                }
            }
            if let Some(m) = message.read().deref() {
                p { class: "help is-danger", "{m}" }
            } else {
                p { class: "help", dangerous_inner_html: "{NBSP}" }
            }
        }
    }
}

fn get_class(message: Signal<Option<String>>) -> String {
    if message.read().is_none() {
        "input".to_owned()
    } else {
        "input is-danger".to_owned()
    }
}
