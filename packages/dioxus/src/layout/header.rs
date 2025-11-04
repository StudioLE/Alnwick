use crate::prelude::*;

#[component]
pub fn HeaderComponent() -> Element {
    let current: Route = use_route();
    let info = current.get_info();
    let breadcrumbs = info.breadcrumbs;
    rsx! {
        section { class: "section",
            nav { class: "breadcrumb",
                aria_label: "breadcrumbs",
                ul {
                    for route in breadcrumbs {
                        BreadcrumbComponent { route: route }
                    }
                }
            }
        }
    }
}

#[component]
fn BreadcrumbComponent(route: Route) -> Element {
    let info = route.get_info();
    rsx! {
        li {
            Link {
                to: route,
                "{info.title}"
            }
        }
    }
}
