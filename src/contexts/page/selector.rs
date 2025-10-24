use crate::pages::settings::component::SettingsMenuComponent;
use crate::prelude::*;
use PageSelector::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum PageSelector {
    #[default]
    Home,
    Podcasts,
    Podcast,
    Settings,
    PlayerSettings,
    AddPodcast,
}

impl PageSelector {
    pub(crate) fn get_info(self) -> PageInfo {
        match self {
            Home => PageInfo {
                title: "Home".to_string(),
                icon: "fa-house".to_string(),
                breadcrumbs: vec![Home, Podcasts],
            },
            Podcasts => PageInfo {
                title: "Player".to_string(),
                icon: "fa-play".to_string(),
                breadcrumbs: vec![Home, Podcasts],
            },
            Podcast => PageInfo {
                title: "Player".to_string(),
                icon: "fa-play".to_string(),
                breadcrumbs: vec![Home, Podcasts, Podcast],
            },
            Settings => PageInfo {
                title: "Player".to_string(),
                icon: "fa-play".to_string(),
                breadcrumbs: vec![Home, Settings],
            },
            PlayerSettings => PageInfo {
                title: "Player".to_string(),
                icon: "fa-play".to_string(),
                breadcrumbs: vec![Home, Settings, PlayerSettings],
            },
            AddPodcast => PageInfo {
                title: "Add".to_string(),
                icon: "fa-plus".to_string(),
                breadcrumbs: vec![Home],
            },
        }
    }

    pub(crate) fn get_component(self) -> Element {
        match self {
            _ => SettingsMenuComponent(),
        }
    }
}
