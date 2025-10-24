use crate::contexts::page::context::PageContext;
use crate::prelude::*;

#[component]
pub(crate) fn SettingsComponent() -> Element {
    rsx! {
        section { class: "section",
            aside { class: "menu",
                p { class: "menu-label", "Personal" }
                ul { class: "menu-list",
                    Item { item: Page::Podcasts }
                }
                p { class: "menu-label", "Entries" }
                ul { class: "menu-list",
                    Item { item: Page::Podcasts }
                    Item { item: Page::Import }
                    Item { item: Page::Export }
                }
            }
        }
    }
}

#[component]
fn Item(item: Page) -> Element {
    let mut context: PageContext = use_context();
    rsx! {
        li {
            a {
                onclick: move |_| context.set(item),
                span { class: "icon has-text-grey-dark",
                    i { class: item.get_icon_classes() }
                }
                span { "{item.get_title()}" }
            }
        }
    }
}
