use crate::contexts::page::context::PageContext;
use crate::prelude::*;

#[component]
pub(super) fn App() -> Element {
    init_contexts();
    let page: PageContext = use_context();
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Link { rel: "stylesheet", href: asset!("/node_modules/bulma/css/bulma.css") }
        document::Link { rel: "stylesheet", href: asset!("/node_modules/@fortawesome/fontawesome-free/css/fontawesome.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/fonts.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/app.css") }
        FloatingActions {}
        div { class: "container is-max-tablet",
            Header {}
            if nav.is_active(Navigation::Settings) {
                Settings {}
            }
            else if nav.is_active(Navigation::Import) {
                Import {}
            }
            else if nav.is_active(Navigation::Chart) {
                Chart {}
            }
            else if nav.is_active(Navigation::Table) {
                Table {}
            }
            else if nav.is_active(Navigation::Goals) {
                Goals {}
            }
            else {
                "Not Implemented"
            }
        }
    }
}

fn init_contexts() {
    let page = PageContext::init();
    use_context_provider(|| page);
}
