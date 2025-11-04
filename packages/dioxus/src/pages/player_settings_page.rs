use crate::prelude::*;

#[component]
pub fn PlayerSettingsPage() -> Element {
    let context: SettingsContext = consume_context();
    rsx! {
        section { class: "section",
            Field::<u32> {
                label: "Skip forward time",
                unit: "s",
                placeholder: "20",
                global_value: context.skip_forward,
                from_string: from_string,
                to_string: to_string,
            },
            Field::<u32> {
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

fn from_string(input: String) -> Result<u32, String> {
    let Ok(value) = input.parse::<u32>() else {
        return Err("Must be a positive integer".to_owned());
    };
    if value > 60 * 60 {
        return Err("Must be less than 3600".to_owned());
    }
    Ok(value)
}

fn to_string(value: Option<u32>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}
