use crate::prelude::*;

#[component]
pub fn AddPodcastPage() -> Element {
    rsx! {
        Page {
            title: "Add podcast",
            subtitle: "From a website or feed",
            MediaObject {
                title: "This page is still under construction",
                subtitle: "It's like 1999 all over again!",
                image_size: ImageSize::_128,
                icon: "fa-person-digging",
            }
        }
    }
}
