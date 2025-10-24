/// Details about a page.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PageInfo {
    pub title: String,
    pub icon: String,
    pub breadcrumbs: Vec<PageInfo>,
}

impl PageInfo {
    /// Get the CSS classes for the page icon.
    pub fn get_icon_classes(&self) -> String {
        format!("fa-solid {}", self.icon)
    }
}
