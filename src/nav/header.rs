use crate::prelude::*;

#[component]
pub(crate) fn Header() -> Element {
    let state: NavigationState = use_context();
    let current = state.get();
    let breadcrumbs = current.get_breadcrumbs();
    rsx! {
        section { class: "section",
            nav { class: "breadcrumb",
                aria_label: "breadcrumbs",
                ul {
                    for breadcrumb in breadcrumbs {
                        Breadcrumb { item: breadcrumb }
                    }
                }
            }
        }
    }
}

#[component]
fn Breadcrumb(item: Navigation) -> Element {
    let mut state: NavigationState = use_context();
    rsx! {
        li {
            a {
                onclick: move |_| state.set(item),
                "{item.get_title()}"
            }
        }
    }
}
