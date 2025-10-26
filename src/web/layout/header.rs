use crate::prelude::*;

#[component]
pub(crate) fn HeaderComponent() -> Element {
    let context: PageContext = use_context();
    let current = context.get();
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
    let mut context: PageContext = use_context();
    let info = selector.get_info();
    rsx! {
        li {
            a {
                onclick: move |_| context.set(selector),
                "{info.title}"
            }
        }
    }
}
