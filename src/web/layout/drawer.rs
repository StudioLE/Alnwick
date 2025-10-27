use crate::prelude::*;

/// A navigation drawer.
/// - `https://m3.material.io/components/navigation-drawer/overview`
#[component]
pub(crate) fn DrawerComponent() -> Element {
    rsx! {
        aside { style: "width: 250px; padding: 1.375em 1.5em;",
            Menu {
                MenuList { label: "Menu",
                    MenuItem { selector: PageSelector::Podcasts },
                    MenuItem { selector: PageSelector::AddPodcast },
                    MenuItem { selector: PageSelector::Settings },
                }
            }
        }
    }
}
