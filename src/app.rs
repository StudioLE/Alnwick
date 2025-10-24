use crate::prelude::*;

/// App is the main component of our app. Components are the building blocks of dioxus apps.
/// Each component is a function that takes some props and returns an Element. In this case,
/// App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages,
/// and autocomplete
#[component]
pub(super) fn App() -> Element {
    init_contexts();
    let nav: NavigationState = use_context();
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
    let entries = EntryState::init();
    let is_entries_empty = entries.is_empty();
    use_context_provider(|| entries);
    let nav = NavigationState::init(is_entries_empty);
    use_context_provider(|| nav);
    let height = HeightState::init();
    use_context_provider(|| height);
}
