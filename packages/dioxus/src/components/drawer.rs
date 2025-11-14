use crate::prelude::*;

/// Properties for [`Drawer`]
#[derive(Clone, Debug, PartialEq, Props)]
pub struct DrawerProps {
    lists: Vec<MenuListProps>,
}

/// A [Material Design 3 navigation drawer](https://m3.material.io/components/navigation-drawer/overview).
#[component]
pub fn Drawer(props: DrawerProps) -> Element {
    rsx! {
        aside { style: "position: fixed; left: 0; top: 0; bottom: 0; padding: 1.375em 1.5em; width: 250px; ",
            Menu {
                lists: props.lists
            }
        }
    }
}
