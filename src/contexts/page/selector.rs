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
                breadcrumbs: vec![Home],
            },
            Podcasts => PageInfo {
                title: "Podcasts".to_string(),
                icon: "fa-users".to_string(),
                breadcrumbs: vec![Home, Podcasts],
            },
            Podcast => PageInfo {
                title: "Podcast".to_string(),
                icon: "fa-user".to_string(),
                breadcrumbs: vec![Home, Podcasts, Podcast],
            },
            Settings => PageInfo {
                title: "Settings".to_string(),
                icon: "fa-cog".to_string(),
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
}
