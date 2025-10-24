use crate::prelude::*;

#[component]
pub(crate) fn FloatingActionsComponent() -> Element {
    let context: PageContext = use_context();
    let actions = [PageSelector::Settings, PageSelector::AddPodcast];
    let actions: Vec<_> = actions
        .into_iter()
        .filter(|&action| !context.is_active(action))
        .enumerate()
        .collect();
    let last = actions.len() - 1;
    rsx! {
        div { class: "fullscreen",
            div { class: "buttons",
                for (i, selector) in actions.into_iter() {
                    FloatingAction {
                        selector,
                        is_large: i == last
                    },
                }
            }
        }
    }
}

#[component]
fn FloatingAction(selector: PageSelector, is_large: bool) -> Element {
    let mut context: PageContext = use_context();
    let info = selector.get_info();
    rsx! {
        button {
            class: get_button_classes(is_large),
            onclick: move |_| context.set(selector),
            span {
                class: "icon",
                i { class: info.get_icon_classes() }
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
