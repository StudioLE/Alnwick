use crate::prelude::*;

#[component]
pub fn Main(title: String, subtitle: Option<String>, children: Element) -> Element {
    rsx! {
        AppBar {
            title, subtitle
        }
        main { class: "container is-max-tablet",
        style: "margin: 90px auto;",
            { children }
        }
    }
}
