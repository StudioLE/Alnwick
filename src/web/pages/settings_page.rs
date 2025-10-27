use crate::prelude::*;
use PageSelector::*;

#[component]
pub(crate) fn SettingsPage() -> Element {
    rsx! {
        section { class: "section",
            Menu {
                MenuList { label: "General",
                    MenuItem { selector: PlayerSettings },
                },
                MenuList { label: "Player",
                    MenuItem { selector: PlayerSettings },
                    MenuItem { selector: PlayerSettings },
                    MenuItem { selector: PlayerSettings },
                }
            }
        }
    }
}
