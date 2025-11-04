use crate::prelude::*;

#[component]
pub fn FloatingActions(children: Element) -> Element {
    rsx! {
        div { class: "fullscreen",
            div { class: "buttons",
                {children}
            }
        }
    }
}

#[component]
pub fn FloatingAction(route: Route, is_large: bool) -> Element {
    let info = route.get_info();
    rsx! {
        Link {
            to: route,
            class: get_button_classes(is_large),
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
