use crate::prelude::*;
use PageSelector::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) enum PageSelector {
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
                title: "Home".to_owned(),
                icon: "fa-house".to_owned(),
                breadcrumbs: vec![Home],
            },
            Podcasts => PageInfo {
                title: "Podcasts".to_owned(),
                icon: "fa-users".to_owned(),
                breadcrumbs: vec![Home, Podcasts],
            },
            Podcast => PageInfo {
                title: "Podcast".to_owned(),
                icon: "fa-user".to_owned(),
                breadcrumbs: vec![Home, Podcasts, Podcast],
            },
            Settings => PageInfo {
                title: "Settings".to_owned(),
                icon: "fa-cog".to_owned(),
                breadcrumbs: vec![Home, Settings],
            },
            PlayerSettings => PageInfo {
                title: "Player".to_owned(),
                icon: "fa-play".to_owned(),
                breadcrumbs: vec![Home, Settings, PlayerSettings],
            },
            AddPodcast => PageInfo {
                title: "Add Podcast".to_owned(),
                icon: "fa-plus".to_owned(),
                breadcrumbs: vec![Home],
            },
        }
    }
}
