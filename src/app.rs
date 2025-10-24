use crate::contexts::page::context::PageContext;
use crate::layout::actions::component::FloatingActionsComponent;
use crate::layout::header::component::HeaderComponent;
use crate::pages::settings::component::SettingsMenuComponent;
use crate::pages::settings::player::component::FieldComponent;
use crate::prelude::*;

#[component]
pub(super) fn App() -> Element {
    init_contexts();
    let context: PageContext = use_context();
    let current = context.get();
    let page = current.get_component();
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Link { rel: "stylesheet", href: asset!("/node_modules/bulma/css/bulma.css") }
        document::Link { rel: "stylesheet", href: asset!("/node_modules/@fortawesome/fontawesome-free/css/fontawesome.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/fonts.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/app.css") }
        FloatingActionsComponent {}
        div { class: "container is-max-tablet",
            HeaderComponent {}
            if current == PageSelector::Home {
                "home page"
            }
            else if current == PageSelector::Podcasts {
                "podcasts page"
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
