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
                        MediaObject {
                            title: feed.podcast.title.clone(),
                            subtitle: "{feed.episodes.len()} episodes · {feed.podcast.id}",
                            image_src: feed.podcast.image.clone(),
                            image_size: ImageSize::_64
                        }
                    }
                }
            }
        }
    }
}
