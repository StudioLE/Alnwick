use crate::prelude::*;

/// A Font Awesome 5 icon with text.
///
/// An implementation of the [Bulma icon element](https://bulma.io/documentation/elements/icon/#icon-text).
#[component]
pub fn Icon(
    class: String,
    text: Option<String>,
    style: Option<IconStyle>,
    size: Option<IconSize>,
    container_size: Option<IconContainerSize>,
) -> Element {
    let class = get_class(class, style, size);
    if let Some(text) = text {
        return rsx! {
            span { class: "icon-text",
                span { class: "icon",
                    i { class: "{class}" }
                }
                span { "{text}" }
            }
        };
    }
    let container_class = if let Some(container_size) = container_size {
        container_size.get_class()
    } else {
        String::new()
    };
    rsx! {
        span { class: "icon {container_class}",
            i { class: "{class}" }
        }
    }
}

fn get_class(class: String, style: Option<IconStyle>, size: Option<IconSize>) -> String {
    let mut classes = Vec::new();
    classes.push(style.unwrap_or_default().get_class());
    if let Some(size) = size {
        classes.push(size.get_class());
    }
    classes.push(class);
    classes.join(" ")
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum IconStyle {
    #[default]
    Solid,
    Regular,
    Light,
    Thin,
    Brands,
}

impl IconStyle {
    #[must_use]
    fn get_class(self) -> String {
        let str = match self {
            IconStyle::Solid => "fa-solid",
            IconStyle::Regular => "fa-regular",
            IconStyle::Light => "fa-light",
            IconStyle::Thin => "fa-thin",
            IconStyle::Brands => "fa-brands",
        };
        str.to_owned()
    }
}

/// - <https://docs.fontawesome.com/web/style/size>
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IconSize {
    // Relative sizing
    // https://docs.fontawesome.com/web/style/size#relative-sizing
    ExtraExtraSmall,
    ExtraSmall,
    Small,
    Large,
    ExtraLarge,
    ExtraExtraLarge,

    // Literal sizing
    // https://docs.fontawesome.com/web/style/size#literal-sizing
    _1x,
    _2x,
    _3x,
    _4x,
    _5x,
    _6x,
    _7x,
    _8x,
    _9x,
    _10x,
}

impl IconSize {
    #[must_use]
    fn get_class(self) -> String {
        let str = match self {
            IconSize::ExtraExtraSmall => "fa-2xs",
            IconSize::ExtraSmall => "fa-xs",
            IconSize::Small => "fa-sm",
            IconSize::Large => "fa-lg",
            IconSize::ExtraLarge => "fa-xl",
            IconSize::ExtraExtraLarge => "fa-2xl",
            IconSize::_1x => "fa-1x",
            IconSize::_2x => "fa-2x",
            IconSize::_3x => "fa-3x",
            IconSize::_4x => "fa-4x",
            IconSize::_5x => "fa-5x",
            IconSize::_6x => "fa-6x",
            IconSize::_7x => "fa-7x",
            IconSize::_8x => "fa-8x",
            IconSize::_9x => "fa-9x",
            IconSize::_10x => "fa-10x",
        };
        str.to_owned()
    }
}

/// - <https://bulma.io/documentation/elements/icon/#sizes>
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum IconContainerSize {
    #[default]
    Default,
    Small,
    Medium,
    Large,
}

impl IconContainerSize {
    #[must_use]
    fn get_class(self) -> String {
        let str = match self {
            IconContainerSize::Default => "",
            IconContainerSize::Small => "is-small",
            IconContainerSize::Medium => "is-medium",
            IconContainerSize::Large => "is-large",
        };
        str.to_owned()
    }
}
