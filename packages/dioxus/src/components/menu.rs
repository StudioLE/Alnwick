use crate::prelude::*;
use Route::*;

#[component]
pub fn Menu(children: Element) -> Element {
    rsx! {
        aside { class: "menu",
            {children}
        }
    }
}

#[component]
pub fn MenuList(label: String, children: Element) -> Element {
    rsx! {
        p { class: "menu-label", "{label}" }
        ul { class: "menu-list",
            {children}
        }
    }
}

#[component]
pub fn MenuItem(route: Route) -> Element {
    let current: Route = use_route();
    let is_active = current.get_info().breadcrumbs.contains(&route);
    let info = route.get_info();
    rsx! {
        li {
            Link {
                to: route,
                class: if is_active { "is-active" } else { "" },
                span { class: "icon has-text-grey-dark",
                    i { class: info.get_icon_classes() }
                }
                span { "{info.title}" }
            }
        }
    }
}
