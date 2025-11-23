use crate::prelude::*;

#[component]
pub fn PodcastPage(id: String) -> Element {
    let resource_id = id.clone();
    let resource = use_resource(move || {
        let resource_id = resource_id.clone();
        async move { get_podcast(resource_id).await }
    });
    match (*resource.read()).clone() {
        None => Loading(),
        Some(Err(error)) => Err(error.into()),
        Some(Ok(None)) => NoPodcast(NoPodcastProps { id }),
        Some(Ok(Some((podcast, episodes)))) => Podcast(PodcastProps { podcast, episodes }),
    }
}

#[component]
fn Loading() -> Element {
    rsx! {
        Page {
                title: "Loading...",
                SkeletonMediaObject {
                    image_size: ImageSize::_128,
                    icon: "fa-image",
                }
                for _i in 0..5 {
                    div { class: "block item pulse-animation",
                        a {
                            SkeletonMediaObject {
                                image_size: ImageSize::_64,
                                icon: "fa-image",
                            }
                        }
                    }
                }
            }
    }
}

#[component]
fn NoPodcast(id: String) -> Element {
    rsx! {
        Page {
                title: "Podcast not found",
                subtitle: "404",
                MediaObject {
                    title: "Unable to find podcast",
                    subtitle: "{id}",
                    image_size: ImageSize::_128,
                    icon: "fa-triangle-exclamation",
                }
            }
    }
}

#[component]
fn Podcast(podcast: PodcastPartial, episodes: Vec<EpisodePartial>) -> Element {
    let subtitle = format!("{} episodes · {}", episodes.len(), podcast.slug);
    rsx! {
        Page {
            title: podcast.title.clone(),
            subtitle: subtitle.clone(),
            MediaObject {
                title: podcast.title.clone(),
                subtitle: subtitle,
                image_src: get_image_url(podcast.image.clone()),
                image_size: ImageSize::_128,
                icon: "fa-image",
            }
            for episode in episodes {
                div { class: "block item",
                    Link {
                        to: Route::Episode { podcast_slug: podcast.slug.clone(), episode_key: episode.primary_key },
                        MediaObject {
                            title: episode.title.clone(),
                            subtitle: get_subtitle(episode.published_at,
                                episode.season,
                                episode.episode,
                                episode.source_duration,
                                episode.kind),
                            image_src: get_image_url(episode.image.clone()).or_else(|| get_image_url(podcast.image.clone())),
                            image_size: ImageSize::_64,
                            icon: "fa-image",
                        }
                    }
                }
            }
        }
    }
}

#[get("/api/podcast/:id")]
async fn get_podcast(
    id: String,
) -> Result<Option<(PodcastPartial, Vec<EpisodePartial>)>, ServerFnError> {
    match SERVICES.metadata.get_podcast(&id).await {
        Ok(option) => Ok(option),
        Err(error) => {
            error!("{error:?}");
            Err(ServerFnError::new(error.to_string()))
        }
    }
}
