use crate::prelude::*;
use Route::*;

#[component]
pub fn SettingsPage() -> Element {
    rsx! {
        section { class: "section",
            Menu {
                MenuList { label: "General",
                    MenuItem { route: PlayerSettings },
                },
                MenuList { label: "Player",
                    MenuItem { route: PlayerSettings },
                    MenuItem { route: PlayerSettings },
                    MenuItem { route: PlayerSettings },
                }
            }
        }
    }
}
