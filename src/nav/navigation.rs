use Navigation::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Navigation {
    Chart,
    Table,
    Add,
    Settings,
    Goals,
    Import,
    Export,
}

impl Navigation {
    pub fn get_title(self) -> String {
        let class = match self {
            Chart => "Lineweights.io",
            Table => "View as a Table",
            Add => "Add a weight or shot",
            Settings => "Settings",
            Goals => "Set Height, BMI, and Goals",
            Import => "Import from File",
            Export => "Export to File",
        };
        class.to_owned()
    }

    pub fn get_icon_classes(self) -> String {
        format!("fa-solid {}", self.get_icon_class())
    }

    pub fn get_icon_class(self) -> String {
        let class = match self {
            Chart => "fa-chart-line",
            Table => "fa-table",
            Add => "fa-plus",
            Settings => "fa-gear",
            Goals => "fa-ruler-vertical",
            Import => "fa-upload",
            Export => "fa-download",
        };
        class.to_owned()
    }

    pub fn get_breadcrumbs(self) -> Vec<Navigation> {
        match self {
            Chart => vec![Chart],
            Table => vec![Chart, Settings, Table],
            Add => vec![Chart, Settings, Add],
            Settings => vec![Chart, Settings],
            Goals => vec![Chart, Settings, Goals],
            Import => vec![Chart, Settings, Import],
            Export => vec![Chart, Settings, Export],
        }
    }
}
