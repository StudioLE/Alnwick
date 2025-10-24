use crate::contexts::page::context::PageContext;
use crate::prelude::*;

#[component]
pub(super) fn PageComponent() -> Element {
    init_contexts();
    let page: PageContext = use_context();
    rsx! {
        if page.is_active(Page::Settings) {
            Settings {}
        }
        else if nav.is_active(Page::Table) {
            Table {}
        }
        else if nav.is_active(Page::Goals) {
            Goals {}
        }
        else {
            "Not Implemented"
        }
    }
}

fn init_contexts() {
    let page = PageContext::init();
    use_context_provider(|| page);
}
