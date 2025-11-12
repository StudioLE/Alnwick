use crate::prelude::*;

/// Global podcasts [context](https://dioxuslabs.com/learn/0.6/reference/context/).
#[derive(Clone, Copy, Debug)]
pub struct PodcastsContext {
    pub loading: Signal<bool>,
    pub podcasts: Signal<HashMap<String, PodcastFeed>>,
}

impl PodcastsContext {
    /// Creates a new instance of the context.
    ///
    /// This should be called at the top of the `App` component.
    pub fn create() {
        let context = Self {
            loading: use_signal(|| true),
            podcasts: use_signal(HashMap::new),
        };
        let mut context = use_context_provider(|| context);
        context.update();
    }

    /// Consume the context from the current scope.
    #[must_use]
    pub fn consume() -> Self {
        consume_context()
    }

    /// Creates a new instance of the context.
    pub fn update(&mut self) {
        let mut context = *self;
        spawn(async move {
            let podcasts = get_podcasts().await.expect("Failed to get podcasts");
            context.podcasts.set(podcasts);
            context.loading.set(false);
        });
    }
}

#[get("/api/podcasts")]
async fn get_podcasts() -> Result<HashMap<String, PodcastFeed>, ServerFnError> {
    let services = ServiceProvider::create()
        .await
        .expect("ServiceProvider should not fail");
    let command = ListCommand::new(services.paths, services.metadata);
    command
        .execute()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
