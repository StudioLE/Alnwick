use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Index,
    #[route("/podcasts/:id")]
    Podcast { id: String },
    #[route("/settings")]
    Settings,
    #[route("/settings/player")]
    PlayerSettings,
    #[route("/add")]
    AddPodcast,
}

impl Route {
    #[must_use]
    pub fn get_info(&self) -> RouteInfo {
        match self {
            Route::Index => RouteInfo {
                title: "Podcasts".to_owned(),
                icon: "fa-podcast".to_owned(),
                breadcrumbs: vec![Route::Index],
                path: "/".to_owned(),
            },
            Route::Podcast { id } => RouteInfo {
                title: "Podcast".to_owned(),
                icon: "fa-user".to_owned(),
                breadcrumbs: vec![Route::Index, Route::Podcast { id: id.clone() }],
                path: format!("/podcasts/{id}"),
            },
            Route::Settings => RouteInfo {
                title: "Settings".to_owned(),
                icon: "fa-cog".to_owned(),
                breadcrumbs: vec![Route::Settings],
                path: "/settings".to_owned(),
            },
            Route::PlayerSettings => RouteInfo {
                title: "Player".to_owned(),
                icon: "fa-play".to_owned(),
                breadcrumbs: vec![Route::Settings, Route::PlayerSettings],
                path: "/settings/player".to_owned(),
            },
            Route::AddPodcast => RouteInfo {
                title: "Add Podcast".to_owned(),
                icon: "fa-plus".to_owned(),
                breadcrumbs: vec![Route::AddPodcast],
                path: "/add".to_owned(),
            },
        }
    }
}

#[component]
fn Index() -> Element {
    IndexPage()
}

#[component]
fn Podcast(id: String) -> Element {
    PodcastPage(PodcastPageProps { id })
}

#[component]
fn Settings() -> Element {
    SettingsPage()
}

#[component]
fn PlayerSettings() -> Element {
    PlayerSettingsPage()
}

#[component]
fn AddPodcast() -> Element {
    rsx! {
        "AddPodcast"
    }
}
