use crate::prelude::*;
use PageSelector::*;

#[component]
pub(crate) fn Menu(children: Element) -> Element {
    rsx! {
        aside { class: "menu",
            {children}
        }
    }
}

#[component]
pub(crate) fn MenuList(label: String, children: Element) -> Element {
    rsx! {
        p { class: "menu-label", "{label}" }
        ul { class: "menu-list",
            {children}
        }
    }
}

#[component]
pub(crate) fn MenuItem(selector: PageSelector) -> Element {
    let mut context: PageContext = consume_context();
    let current = context.get();
    let is_active = current.get_info().breadcrumbs.contains(&selector);
    let info = selector.get_info();
    rsx! {
        li {
            a {
                class: if is_active { "is-active" } else { "" },
                onclick: move |_| context.set(selector),
                span { class: "icon has-text-grey-dark",
                    i { class: info.get_icon_classes() }
                }
                span { "{info.title}" }
            }
        }
    }
}
