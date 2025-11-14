use crate::prelude::*;

/// A simple breadcrumbs component to improve your navigation experience.
///
/// An implementation of the [Bulma breadcrumb component](https://bulma.io/documentation/components/breadcrumb/).
#[component]
pub fn Breadcrumbs(route: Route) -> Element {
    let info = route.get_info();
    rsx! {
        nav { class: "breadcrumb",
            aria_label: "breadcrumbs",
            ul {
                for route in info.breadcrumbs {
                    BreadcrumbItem { route: route }
                }
            }
        }
    }
}

#[component]
fn BreadcrumbItem(route: Route) -> Element {
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
