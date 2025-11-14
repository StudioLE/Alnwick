use crate::prelude::*;

#[component]
pub fn IndexPage() -> Element {
    let context = PodcastsContext::consume();
    if *context.loading.read() {
        return rsx! {
            "Loading..."
        };
    }
    let podcasts = context.podcasts.read();
    if podcasts.is_empty() {
        return rsx! {
            "No podcasts found"
        };
    }
    rsx! {

        Main {
            title: "Hello, world!",
            subtitle: "This is a subtitle!",
            for feed in podcasts.values() {
                div { class: "block item",
                    Link {
                        to: Route::Podcast { id: feed.podcast.id.clone() },
                        article { class: "media",
                            figure { class: "media-left",
                                p { class: "image is-64x64",
                                    if let Some(url) = &feed.podcast.image {
                                        img { src: "{url}" }
                                    }
                                }
                            }
                            div {
                                class: "media-content",
                                style: "align-self: center;",
                                p { class: "title",
                                    "{feed.podcast.title} "
                                }
                                p { class: "subtitle",
                                    "{feed.episodes.len()} episodes · {feed.podcast.id}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
