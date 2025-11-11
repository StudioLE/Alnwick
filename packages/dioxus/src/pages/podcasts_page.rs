use crate::prelude::*;

#[component]
pub fn PodcastsPage() -> Element {
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
        for podcast in podcasts.iter() {
            div { class: "block",
                Link {
                    to: Route::Podcast { id: podcast.id.clone() },
                    article { class: "media",
                        figure { class: "media-left",
                            p { class: "image is-96x96",
                                if let Some(url) = &podcast.image_url {
                                    img { src: "{url}" }
                                }
                            }
                        }
                        div {
                            class: "media-content",
                            style: "align-self: center;",
                            p { class: "subtitle is-5",
                                "{podcast.title} "
                                span { class: "tag",
                                    "{podcast.episodes.len()}"
                                }
                                " "
                                span { class: "tag",
                                    "{podcast.id}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
