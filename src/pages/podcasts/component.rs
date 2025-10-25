use crate::prelude::*;

#[component]
pub(crate) fn PodcastsPage() -> Element {
    let resource = use_resource(get_podcasts);
    let Some(result) = &*resource.read() else {
        return rsx! {
            "Loading..."
        };
    };
    match result {
        Ok(podcasts) => rsx! {
            for podcast in podcasts {
                div {
                    "{podcast.title}"
                }
            }
        },
        Err(e) => rsx! {
            "{e}"
        },
    }
}

#[server]
pub async fn get_podcasts() -> Result<Vec<Podcast>, ServerFnError<String>> {
    let services = ServiceProvider::create()
        .await
        .expect("ServiceProvider should not fail");
    let command = ListCommand::new(services.paths, services.http, services.metadata);
    command
        .execute()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
