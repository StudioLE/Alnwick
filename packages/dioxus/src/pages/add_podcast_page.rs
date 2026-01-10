use crate::prelude::*;

/// Page for adding a new podcast from an RSS feed URL.
#[allow(clippy::absolute_paths)]
#[component]
pub fn AddPodcastPage() -> Element {
    let slug: Signal<Option<Slug>> = use_signal(|| None);
    let url: Signal<Option<UrlWrapper>> = use_signal(|| None);
    let mut is_loading = use_signal(|| false);
    let mut error_message: Signal<Option<String>> = use_signal(|| None);
    let nav = navigator();
    let is_valid = use_memo(move || slug.read().is_some() && url.read().is_some());
    let on_submit = move |event: FormEvent| async move {
        event.prevent_default();
        let (Some(slug_val), Some(url_val)) = (slug.cloned(), url.cloned()) else {
            return;
        };
        is_loading.set(true);
        error_message.set(None);
        match add_podcast(slug_val.to_string(), url_val.to_string()).await {
            Ok(_response) => {
                nav.push(Route::Podcast { slug: slug_val });
            }
            Err(e) => {
                error_message.set(Some(e.to_string()));
                is_loading.set(false);
            }
        }
    };
    rsx! {
        Page {
            title: "Add podcast",
            subtitle: "From an RSS feed URL",
            form {
                onsubmit: on_submit,
                Field::<Slug> {
                    label: "Slug",
                    placeholder: "my-podcast",
                    global_value: slug,
                    from_string: slug_from_string,
                    to_string: slug_to_string,
                }
                Field::<UrlWrapper> {
                    label: "Feed URL",
                    placeholder: "https://example.com/feed.xml",
                    global_value: url,
                    from_string: url_from_string,
                    to_string: url_to_string,
                }
                if let Some(msg) = error_message.read().as_ref() {
                    article { class: "message is-danger",
                        div { class: "message-body", "{msg}" }
                    }
                }
                div { class: "field",
                    div { class: "control",
                        button {
                            class: "button is-primary",
                            r#type: "submit",
                            disabled: !is_valid() || is_loading(),
                            if is_loading() {
                                "Adding..."
                            } else {
                                "Add Podcast"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn slug_from_string(input: String) -> Result<Slug, String> {
    Slug::from_str(&input).map_err(|e| e.to_string())
}

fn slug_to_string(value: Option<Slug>) -> String {
    value.map(|v| v.to_string()).unwrap_or_default()
}

fn url_from_string(input: String) -> Result<UrlWrapper, String> {
    UrlWrapper::from_str(&input).map_err(|e| e.to_string())
}

fn url_to_string(value: Option<UrlWrapper>) -> String {
    value.map(|v| v.to_string()).unwrap_or_default()
}

#[post("/api/podcasts")]
async fn add_podcast(slug: String, url: String) -> Result<AddResponse, ServerFnError> {
    let slug = Slug::from_str(&slug).map_err(|e| ServerFnError::new(e.to_string()))?;
    let feed_url = UrlWrapper::from_str(&url).map_err(|e| ServerFnError::new(e.to_string()))?;
    let request = AddRequest { slug, feed_url };
    let handler = get_add_handler().await;
    match handler.execute(&request).await {
        Ok(response) => Ok(response),
        Err(e) => {
            error!("{e:?}");
            Err(ServerFnError::new(e.to_string()))
        }
    }
}
