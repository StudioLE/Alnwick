use crate::prelude::dioxus_fullstack::Streaming;
use crate::prelude::*;
use dioxus::fullstack::JsonEncoding;
#[cfg(feature = "server")]
use tokio::sync::broadcast::error::RecvError;

/// Global settings [context](https://dioxuslabs.com/learn/0.6/reference/context/).
#[derive(Clone, Copy)]
pub struct DownloadContext {
    hash_map: Signal<HashMap<DownloadRequest, DownloadStatus>>,
}

impl DownloadContext {
    /// Creates a new instance of the context.
    ///
    /// This should be called at the top of the `App` component.
    pub fn create() {
        let context = Self {
            hash_map: Signal::new(HashMap::default()),
        };
        use_context_provider(|| context);
    }

    /// Consume the context from the current scope.
    #[must_use]
    pub fn consume() -> Self {
        consume_context()
    }

    /// Consume the context from the current scope.
    #[must_use]
    pub fn use_context() -> Self {
        use_context()
    }

    /// Get the download status for the request.
    #[must_use]
    pub fn get(&self, request: DownloadRequest) -> Option<DownloadStatus> {
        self.hash_map.read().get(&request).cloned()
    }

    /// Set the download status for the request.
    fn set(&mut self, request: DownloadRequest, status: DownloadStatus) {
        self.hash_map.write().insert(request, status);
    }

    /// Queue a download request.
    pub async fn download(&mut self, request: DownloadRequest) {
        trace!("Downloading {request}");
        self.set(request, DownloadStatus::Queued);
        // TODO: Add handling for error
        queue_download(request.podcast, request.episode)
            .await
            .expect("should be able to queue download");
    }

    /// Watch the event stream and update the download status.
    #[allow(irrefutable_let_patterns)]
    pub fn watch_events() {
        trace!("Watching for events");
        use_coroutine(move |_: UnboundedReceiver<()>| async move {
            trace!("Starting event stream");
            let Ok(mut stream) = event_stream().await else {
                error!("Failed to create event stream");
                return;
            };
            trace!("Started event stream");
            while let Some(result) = stream.next().await {
                let event = match result {
                    Ok(event) => event,
                    Err(e) => {
                        error!("Error reading event stream: {e:?}");
                        break;
                    }
                };
                trace!("Received event: {event:?}");
                let CommandRequest::Download(request) = event.get_request() else {
                    continue;
                };
                let status = match event.get_kind() {
                    EventKind::Queued => DownloadStatus::Queued,
                    EventKind::Executing => DownloadStatus::Downloading,
                    EventKind::Succeeded => {
                        let Some(CommandSuccess::Download(response)) = event.get_success() else {
                            warn!("Expected sucessful download event to have a success value");
                            continue;
                        };
                        DownloadStatus::Succeeded(response.file_path.clone())
                    }
                    EventKind::Failed => DownloadStatus::Failed,
                };
                let mut context = DownloadContext::consume();
                context.set(*request, status);
            }
            warn!("Event stream closed");
        });
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DownloadStatus {
    Queued,
    Downloading,
    Failed,
    Succeeded(PathBuf),
}

#[get("/api/download/:podcast/:episode")]
async fn queue_download(podcast: PodcastKey, episode: EpisodeKey) -> Result<(), ServerFnError> {
    let request = DownloadRequest::new(podcast, episode);
    trace!("Adding to queue {request}");
    let runner = get_runner().await;
    if let Err(error) = runner.queue_request(request).await {
        error!("{error:?}");
        Err(ServerFnError::new(error.to_string()))
    } else {
        Ok(())
    }
}

#[get("/api/events")]
#[allow(clippy::unused_async)]
async fn event_stream() -> Result<Streaming<CommandEvent, JsonEncoding>> {
    trace!("Spawning event stream");
    let streaming = Streaming::spawn(|sender| async move {
        trace!("Subscribing to events");
        let mut receiver = subscribe_to_events().await;
        loop {
            trace!("Waiting for event");
            let event = match receiver.recv().await {
                Err(RecvError::Lagged(count)) => {
                    warn!("Event relay missed {count} events due to lagging");
                    continue;
                }
                Err(RecvError::Closed) => {
                    error!("Event pipe was closed. Event relay can't proceed.");
                    break;
                }
                Ok(event) => event,
            };
            trace!("Relaying event: {event:?}");
            if let Err(e) = sender.unbounded_send(event) {
                error!("Event relay failed to send event: {e:?}");
                break;
            }
        }
    });
    Ok(streaming)
}
