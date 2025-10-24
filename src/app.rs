use dioxus::document::Link;
use crate::contexts::page::context::PageContext;
use crate::layout::actions::component::FloatingActionsComponent;
use crate::layout::header::component::HeaderComponent;
use crate::pages::settings::component::SettingsMenuComponent;
use crate::pages::settings::player::component::FieldComponent;
use crate::prelude::*;
use crate::prelude::document::Stylesheet;

static BULMA_CSS: Asset = asset!("/node_modules/bulma/css/bulma.css");
static FONTAWESOME_CSS: Asset = asset!("/node_modules/@fortawesome/fontawesome-free/css/fontawesome.css");
static FONTS_CSS: Asset = asset!("/assets/fonts.css");
static APP_CSS: Asset = asset!("/assets/app.css");
static FAVICON: Asset = asset!("/assets/favicon.ico");

#[component]
pub(super) fn App() -> Element {
    init_contexts();
    let context: PageContext = use_context();
    let current = context.get();
    rsx! {
        Head {}
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
#[component]
fn Head() -> Element {
    rsx! {
        Link { rel: "icon", href: FAVICON }
        Stylesheet { href: BULMA_CSS }
        Stylesheet { href: FONTAWESOME_CSS }
        Stylesheet { href: FONTS_CSS }
        Stylesheet { href: APP_CSS }
    }
}

fn init_contexts() {
    let page = PageContext::init();
    let settings = SettingsContext::init();
    use_context_provider(|| page);
    use_context_provider(|| settings);
}
