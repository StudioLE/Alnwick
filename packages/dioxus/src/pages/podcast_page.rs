use crate::prelude::*;

#[component]
pub fn PodcastPage(id: String) -> Element {
    let context = PodcastsContext::consume();
    if *context.loading.read() {
        return rsx! {
            "Loading..."
        };
    }
    let Some(feed) = context.podcasts.get(&id) else {
        return rsx! {
            "Unable to find podcast: {id}"
        };
    };
    let subtitle = format!("{} episodes · {}", feed.episodes.len(), feed.podcast.id);
    rsx! {
        Main {
            title: feed.podcast.title.clone(),
            subtitle: subtitle.clone(),
            MediaObject {
                title: feed.podcast.title.clone(),
                subtitle: subtitle,
                image_src: feed.podcast.image.clone(),
                image_size: ImageSize::_128
            }
            for episode in feed.episodes.iter() {
                div { class: "block item",
                    Link {
                        to: Route::Episode { podcast_id: feed.podcast.id.clone(), episode_id: episode.id },
                        MediaObject {
                            title: episode.title.clone(),
                            subtitle: episode.get_subtitle(),
                            image_src: episode.image.clone().or_else(|| feed.podcast.image.clone()),
                            image_size: ImageSize::_64
                        }
                    }
                }
            }
        }
    }
}
