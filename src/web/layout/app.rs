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
                    if current == PageSelector::Home {
                        "home page"
                    }
                    else if current == PageSelector::Podcasts {
                        PodcastsPage {}
                    }
                    else if current == PageSelector::Podcast {
                        "podcast page"
                    }
                    else if current == PageSelector::Settings {
                        SettingsPage {}
                    }
                    else if current == PageSelector::PlayerSettings {
                        FieldComponent {}
                    }
                    else if current == PageSelector::AddPodcast {
                        "add podcast page"
                    }
                    else {
                        "Page not found"
                    }
                }
            }
        }
    }
}

fn init_contexts() {
    let page = PageContext::init();
    let settings = SettingsContext::init();
    use_context_provider(|| page);
    use_context_provider(|| settings);
}
