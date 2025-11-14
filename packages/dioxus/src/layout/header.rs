use crate::prelude::*;

#[component]
pub fn HeaderComponent() -> Element {
    let route: Route = use_route();
    rsx! {
        section { class: "section",
            Breadcrumbs { route }
        }
    }
}
