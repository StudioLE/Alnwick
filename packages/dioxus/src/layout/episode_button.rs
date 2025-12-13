use crate::contexts::DownloadContext;
use crate::prelude::*;

#[component]
pub fn EpisodeButton(podcast: PodcastPartial, episode: EpisodePartial) -> Element {
    let request = DownloadRequest::new(podcast.primary_key, episode.primary_key);
    if let Some(path) = episode.file_sub_path.clone() {
        return rsx! {
            PlayButton { request, path: PathBuf::from(path) }
        };
    }
    let context = DownloadContext::use_context();
    if let Some(status) = context.get(request) {
        rsx! {
            StatusButton { request, status }
        }
    } else {
        rsx! {
            DownloadButton { request }
        }
    }
}

#[component]
fn PlayButton(request: DownloadRequest, path: PathBuf) -> Element {
    rsx! {
        a { class: "button is-small",
            onclick: move |e: Event<MouseData>| {
                e.stop_propagation();
                e.prevent_default();
                trace!(%request, path = %path.display(), "Play button clicked");
            },
            Icon {
                class: "fa-play",
            }
        }
    }
}

#[component]
fn DownloadButton(request: DownloadRequest) -> Element {
    rsx! {
        a { class: "button is-small",
            onclick: move |e: Event<MouseData>| {
                async move {
                    e.stop_propagation();
                    e.prevent_default();
                    let mut context = DownloadContext::consume();
                    context.download(request).await;
                    trace!(%request, "Download button clicked");
                }
            },
            Icon {
                class: "fa-download",
            }
        }
    }
}

#[component]
fn StatusButton(request: DownloadRequest, status: DownloadStatus) -> Element {
    let icon = match status {
        DownloadStatus::Queued => "fa-solid fa-circle-notch fa-spin",
        DownloadStatus::Downloading => "fa-solid fa-rotate fa-spin",
        DownloadStatus::Failed => "fa-solid fa-exclamation-triangle",
        DownloadStatus::Succeeded(path) => {
            return rsx! {
                PlayButton { request, path }
            };
        }
    };
    rsx! {
        a { class: "button is-small",
            Icon {
                class: "{icon}",
            }
        }
    }
}
