use crate::prelude::*;

#[component]
pub(crate) fn FloatingActions(children: Element) -> Element {
    rsx! {
        div { class: "fullscreen",
            div { class: "buttons",
                {children}
            }
        }
    }
}

#[component]
pub(crate) fn FloatingAction(selector: PageSelector, is_large: bool) -> Element {
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
