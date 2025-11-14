use crate::prelude::*;

#[component]
pub fn App() -> Element {
    SettingsContext::create();
    PodcastsContext::create();
    rsx! {
        Router::<Route> {}
    }
}

#[component]
pub fn Layout() -> Element {
    rsx! {
        HeadComponent {}
        Drawer {
            lists: vec![
                MenuListProps {
                    label: "Menu".to_owned(),
                    routes: vec![Route::Index, Route::AddPodcast, Route::Settings]
                }
            ]
        },
        Bar {}
        main { class: "container is-max-tablet",
            style: "margin: 90px auto;",
            Outlet::<Route> {}
        }
        FloatingActions {
            routes: vec![Route::AddPodcast]
        }
        footer { style: "
            position: fixed;
            left: 0;
            right: 0;
            bottom: 0;
            background-color: var(--bulma-body-background-color);",
            Tabs {
                routes: vec![Route::Index, Route::AddPodcast, Route::Settings]
            }
        }
    }
}
