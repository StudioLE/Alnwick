use crate::prelude::*;

/// A [Material Design 3 app bar](https://m3.material.io/components/app-bars/overview).
#[component]
pub fn AppBar(title: String, subtitle: Option<String>) -> Element {
    let current: Route = use_route();
    let breadcrumbs = current.get_info().breadcrumbs;
    let previous = if breadcrumbs.len() > 1 {
        breadcrumbs.get(breadcrumbs.len() - 2)
    } else {
        None
    };
    rsx! {
        header { style: "
            position: fixed;
            left: 0;
            right: 0;
            top: 0;
            z-index: 1;
            background-color: var(--bulma-body-background-color)",
            class: "container is-max-tablet",
            div { style: "
                margin: var(--bulma-block-spacing) 0;
                display: flex;
                align-items: center;
                gap: 1rem;",
                if let Some(previous) = previous {
                    div {
                        Link {
                            to: previous.clone(),
                            span {
                                class: "icon is-medium",
                                i { class: "fas fa-arrow-left fa-lg" }
                            }
                        }
                    }
                }
                div {
                    p { class: "title",
                        "{title} "
                    }
                    if let Some(subtitle) = subtitle {
                        p { class: "subtitle",
                            "{subtitle}"
                        }
                    }
                }
            }
        }
    }
}
