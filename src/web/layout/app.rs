use crate::prelude::*;

#[component]
pub fn App() -> Element {
    init_contexts();
    let context: PageContext = use_context();
    let current = context.get();
    rsx! {
        HeadComponent {}
        FloatingActions {
            FloatingAction {
                selector: PageSelector::AddPodcast,
                is_large: true,
            }
        }
        div { style: "display: flex; height: 100vh;",
            DrawerComponent {},
            main { style: "flex: 1;",
                div { class: "container is-max-tablet",
                    HeaderComponent {}
                    {current.get_component()}
                }
            }
        }
    }
}

fn init_contexts() {
    let page = PageContext::init();
    let settings = SettingsContext::new();
    use_context_provider(|| page);
    use_context_provider(|| settings);
}
