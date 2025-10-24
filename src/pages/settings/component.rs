use crate::prelude::*;
use PageSelector::*;

#[component]
pub(crate) fn SettingsMenuComponent() -> Element {
    rsx! {
        section { class: "section",
            aside { class: "menu",
                p { class: "menu-label", "Personal" }
                ul { class: "menu-list",
                    ItemComponent { selector: PlayerSettings }
                }
                p { class: "menu-label", "Entries" }
                ul { class: "menu-list",
                    ItemComponent { selector: PlayerSettings }
                    ItemComponent { selector: PlayerSettings }
                    ItemComponent { selector: PlayerSettings }
                }
            }
        }
    }
}

#[component]
fn ItemComponent(selector: PageSelector) -> Element {
    let mut context: PageContext = consume_context();
    let info = selector.get_info();
    rsx! {
        li {
            a {
                onclick: move |_| context.set(selector),
                span { class: "icon has-text-grey-dark",
                    i { class: info.get_icon_classes() }
                }
                span { "{info.title}" }
            }
        }
    }
}
