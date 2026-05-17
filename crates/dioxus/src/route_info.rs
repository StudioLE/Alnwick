use crate::prelude::*;

/// Details about a page.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RouteInfo {
    pub title: String,
    pub icon: String,
    pub previous: Option<Route>,
    pub breadcrumbs: Vec<Route>,
    pub path: String,
}
