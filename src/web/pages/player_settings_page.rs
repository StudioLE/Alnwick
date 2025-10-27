use crate::prelude::*;

#[component]
pub(crate) fn PlayerSettingsPage() -> Element {
    let context: SettingsContext = consume_context();
    rsx! {
        section { class: "section",
            Field {
                label: "Skip forward time",
                unit: "s",
                placeholder: "20",
                global_value: context.skip_forward,
                from_string: from_string,
                to_string: to_string,
            },
            Field {
                label: "Skip back time",
                unit: "s",
                placeholder: "20",
                global_value: context.skip_back,
                from_string: from_string,
                to_string: to_string,
            },
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
    value.map(|value| value.to_string()).unwrap_or_default()
}
