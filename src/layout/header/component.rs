use crate::prelude::*;

#[component]
pub(crate) fn HeaderComponent() -> Element {
    let state: PageContext = use_context();
    let current = state.get();
    let info = current.get_info();
    let breadcrumbs = info.breadcrumbs;
    rsx! {
        section { class: "section",
            nav { class: "breadcrumb",
                aria_label: "breadcrumbs",
                ul {
                    for selector in breadcrumbs {
                        BreadcrumbComponent { selector: selector }
                    }
                }
            }
        }
    }
}

#[component]
fn BreadcrumbComponent(selector: PageSelector) -> Element {
    let mut state: PageContext = use_context();
    let info = selector.get_info();
    rsx! {
        li {
            a {
                onclick: move |_| state.set(selector),
                "{info.title}"
            }
        }
    }
}
