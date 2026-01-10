use crate::prelude::*;
use std::ops::Deref;

const NBSP: char = '\u{00A0}';

#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Props, Clone, PartialEq)]
pub struct FieldProps<T>
where
    T: 'static + Clone + PartialEq,
{
    /// Label displayed above the field.
    pub label: String,
    /// Placeholder text when the field is empty.
    pub placeholder: String,
    /// Global value.
    pub global_value: Signal<Option<T>>,
    /// Convert from input text to a value.
    pub from_string: fn(String) -> Result<T, String>,
    /// Convert from a value to input text.
    pub to_string: fn(Option<T>) -> String,
    /// Unit displayed at the end of the field.
    #[props(default)]
    pub unit: Option<String>,
}

#[component]
pub fn Field<T>(mut props: FieldProps<T>) -> Element
where
    T: 'static + Clone + PartialEq,
{
    let initial_value = (props.to_string)(props.global_value.cloned());
    let mut field_value = use_signal(|| initial_value);
    let mut message: Signal<Option<String>> = use_signal(|| None);
    let from_string = props.from_string;
    let mut validate = move |input: String| {
        if input.is_empty() {
            props.global_value.set(None);
            message.set(None);
            return;
        }
        match from_string(input) {
            Ok(value) => {
                props.global_value.set(Some(value));
                message.set(None);
            }
            Err(e) => {
                props.global_value.set(None);
                message.set(Some(e));
            }
        }
    };
    let has_unit = props.unit.is_some();
    rsx! {
        div { class: "field",
            label { class: "label", "{props.label}" }
            div { class: if has_unit { "field has-addons" } else { "control" },
                p { class: "control",
                    input {
                        oninput: move |event| {
                            event.prevent_default();
                            let input_value = event.value();
                            field_value.set(input_value.clone());
                            validate(input_value);
                        },
                        class: get_class(message),
                        r#type: "text",
                        placeholder: props.placeholder,
                        value: field_value,
                    }
                }
                if let Some(unit) = &props.unit {
                    p { class: "control",
                        a { class: "button is-static", "{unit}" }
                    }
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
