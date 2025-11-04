use crate::prelude::*;

#[component]
pub fn App() -> Element {
    init_contexts();
    rsx! {
        Router::<Route> {}
    }
}

#[component]
pub fn Layout() -> Element {
    rsx! {
        HeadComponent {}
        FloatingActions {
            FloatingAction {
                route: Route::AddPodcast,
                is_large: true,
            }
        }
        div { style: "display: flex; height: 100vh;",
            DrawerComponent {},
            main { style: "flex: 1;",
                div { class: "container is-max-tablet",
                    HeaderComponent {}
                    Outlet::<Route> {}
                }
            }
        }
    }
}

fn init_contexts() {
    let settings = SettingsContext::new();
    use_context_provider(|| settings);
}
