use crate::prelude::*;

#[component]
pub(crate) fn FloatingActions() -> Element {
    let state: NavigationState = use_context();
    let actions = [Navigation::Settings, Navigation::Add];
    let actions: Vec<_> = actions
        .into_iter()
        .filter(|&action| !state.is_active(action))
        .enumerate()
        .collect();
    let last = actions.len() - 1;
    rsx! {
        div { class: "fullscreen",
            div { class: "buttons",
                for (i, action) in actions.into_iter() {
                    FloatingAction {
                        action,
                        is_large: i == last
                    },
                }
            }
        }
    }
}

#[component]
fn FloatingAction(action: Navigation, is_large: bool) -> Element {
    let mut state: NavigationState = use_context();
    rsx! {
        button {
            class: get_button_classes(is_large),
            onclick: move |_| state.set(action),
            span {
                class: "icon",
                i { class: action.get_icon_classes() }
            }
        }
    }
}

fn get_button_classes(is_large: bool) -> String {
    let mut output = "button is-primary".to_owned();
    if is_large {
        output.push_str(" is-large");
    }
    output
}
