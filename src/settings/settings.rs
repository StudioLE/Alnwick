use crate::prelude::*;

#[component]
pub(crate) fn Settings() -> Element {
    rsx! {
        section { class: "section",
            aside { class: "menu",
                p { class: "menu-label", "Personal" }
                ul { class: "menu-list",
                    Item { item: Navigation::Goals }
                }
                p { class: "menu-label", "Entries" }
                ul { class: "menu-list",
                    Item { item: Navigation::Table }
                    Item { item: Navigation::Import }
                    Item { item: Navigation::Export }
                }
            }
        }
    }
}

#[component]
fn Item(item: Navigation) -> Element {
    let mut state: NavigationState = use_context();
    rsx! {
        li {
            a {
                onclick: move |_| state.set(item),
                span { class: "icon has-text-grey-dark",
                    i { class: item.get_icon_classes() }
                }
                span { "{item.get_title()}" }
            }
        }
    }
}
