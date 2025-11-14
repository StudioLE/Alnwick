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
    rsx! {
        Main {
            title: episode.title.clone(),
            subtitle: subtitle.clone(),
            div { class: "block",
                header { class: "media",
                    figure { class: "media-left",
                        p { class: "image is-128x128",
                            if let Some(url) = &episode.image {
                                img { src: "{url}" }
                            } else {
                                if let Some(url) = &feed.podcast.image {
                                    img { src: "{url}" }
                                }
                            }
                        }
                    }
                    div {
                        class: "media-content",
                        style: "align-self: center;",
                        p { class: "title",
                            "{episode.title} "
                        }
                        p { class: "subtitle",
                            "{subtitle}"
                        }
                    }
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
