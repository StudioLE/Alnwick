use crate::prelude::document::{Link, Stylesheet};
use crate::prelude::*;

static BULMA_CSS: Asset = asset!("/node_modules/bulma/css/bulma.css");
static FONTAWESOME_CSS: Asset =
    asset!("/node_modules/@fortawesome/fontawesome-free/css/fontawesome.css");
static FONTS_CSS: Asset = asset!("/assets/fonts.css");
static APP_CSS: Asset = asset!("/assets/app.css");
static FAVICON: Asset = asset!("/assets/favicon.ico");

#[component]
pub fn HeadComponent() -> Element {
    rsx! {
        Link { rel: "icon", href: FAVICON }
        Stylesheet { href: BULMA_CSS }
        Stylesheet { href: FONTAWESOME_CSS }
        Stylesheet { href: FONTS_CSS }
        Stylesheet { href: APP_CSS }
    }
}
