use crate::prelude::*;
use html2text::config::plain;

#[component]
pub fn EpisodePage(podcast_id: String, episode_id: Uuid) -> Element {
    let context = PodcastsContext::consume();
    if *context.loading.read() {
        return rsx! {
            "Loading..."
        };
    }
    let Some(feed) = context.podcasts.get(&podcast_id) else {
        return rsx! {
            "Unable to find podcast: {podcast_id}"
        };
    };
    let Some(episode) = feed
        .episodes
        .iter()
        .find(|episode| episode.id == episode_id)
    else {
        return rsx! {
            "Unable to find episode: {episode_id}"
        };
    };
    let description = episode.get_description();
    let subtitle = episode.get_subtitle();
    let image = episode.image.clone().or_else(|| feed.podcast.image.clone());
    rsx! {
        Page {
            title: episode.title.clone(),
            subtitle: subtitle.clone(),
            div { class: "block",
                MediaObject {
                    title: episode.title.clone(),
                    subtitle: subtitle,
                    image_src: image,
                    image_size: ImageSize::_128
                }
                if let Some(description) = description {
                    article {
                        pre {
                            "{description}"
                        }
                    }
                }
            }
        }
    }
}
