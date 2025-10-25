use crate::contexts::page::context::PageContext;
use crate::layout::actions::FloatingActionsComponent;
use crate::layout::head::HeadComponent;
use crate::layout::header::HeaderComponent;
use crate::pages::podcasts::component::PodcastsPage;
use crate::pages::settings::component::SettingsMenuComponent;
use crate::pages::settings::player::component::FieldComponent;
use crate::prelude::*;

#[component]
pub fn App() -> Element {
    init_contexts();
    let context: PageContext = use_context();
    let current = context.get();
    rsx! {
        HeadComponent {}
        FloatingActionsComponent {}
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
                SettingsMenuComponent {}
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

fn init_contexts() {
    let page = PageContext::init();
    let settings = SettingsContext::init();
    use_context_provider(|| page);
    use_context_provider(|| settings);
}
