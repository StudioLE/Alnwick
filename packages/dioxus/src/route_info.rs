use crate::prelude::*;

/// Details about a page.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RouteInfo {
    pub title: String,
    pub icon: String,
    pub breadcrumbs: Vec<Route>,
    pub path: String,
}

impl RouteInfo {
    /// Get the CSS classes for the page icon.
    #[must_use]
    pub fn get_icon_classes(&self) -> String {
        format!("fa-solid {}", self.icon)
    }
}
