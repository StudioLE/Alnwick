use crate::prelude::*;

#[component]
pub fn Page(title: String, subtitle: Option<String>, children: Element) -> Element {
    rsx! {
        AppBar {
            title, subtitle
        }
        main { class: "container is-max-tablet",
            style: "
            margin-top: calc(var(--app-bar-height) + var(--bulma-block-spacing));
            margin-left: auto;
            margin-bottom: calc(var(--nav-bar-height) + var(--bulma-block-spacing));
            margin-right: auto;",
            { children }
        }
    }
}
