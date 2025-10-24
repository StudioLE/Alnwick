use super::*;
use crate::prelude::*;
use ImportError::*;

#[allow(clippy::absolute_paths)]
#[component]
pub fn Import() -> Element {
    let state: EntryState = use_context();
    let nav: NavigationState = use_context();
    rsx! {
        section { class: "section",
            h1 { class: "title", "Import data from Shotsy" }
            h2 { class: "subtitle",
                "This app is designed to visualize the data you already have stored in Shotsy."
            }
            div { class: "content",
                ol { class: "block",
                    li { "Open the Shotsy app" }
                    li {"Go to ", code { "Settings > Export Data > Export Data to CSV" } }
                    li { "Save the ", code { "shotsy_export.csv" }, " file to your device" }
                    li { "Import it with the button below" }
                }
            }
            div { class: "file",
                label { class: "file-label",
                    input {
                        class: "file-input",
                        r#type: "file",
                        accept: ".csv",
                        multiple: false,
                        onchange: move |event| onchange(event, state, nav),
                    }
                    span { class: "file-cta",
                        span { class: "file-label", " Choose a file… " }
                    }
                }
            }
        }
    }
}

async fn onchange(event: Event<FormData>, mut state: EntryState, mut nav: NavigationState) {
    let Some(csv) = read_file(event)
        .await
        .handle_error(|e| warn!("Failed to read file: {e:?}"))
    else {
        return;
    };
    let data = ShotsyData::from_csv(&csv);
    let entries = ShotsyData::to_entries(data);
    let Some(collection) =
        EntryCollection::new(entries).handle_error(|e| warn!("Failed to determine range: {e:?}"))
    else {
        return;
    };
    state.set(collection);
    nav.set(Navigation::Chart);
}

async fn read_file(evt: Event<FormData>) -> Result<String, ImportError> {
    let Some(file_engine) = evt.files() else {
        return Err(NoFileEngine);
    };
    let files = file_engine.files();
    if files.is_empty() {
        return Err(NoFiles);
    }
    if files.len() > 1 {
        return Err(MultipleFiles);
    }
    let file = files.first().expect("should be exactly one file");
    file_engine
        .read_file_to_string(file)
        .await
        .ok_or(FailedToRead)
}

#[derive(Debug)]
pub enum ImportError {
    NoFileEngine,
    MultipleFiles,
    NoFiles,
    FailedToRead,
}

impl Display for ImportError {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for ImportError {}
