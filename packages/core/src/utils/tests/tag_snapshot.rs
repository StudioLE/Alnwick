use crate::prelude::*;
use lofty::prelude::TaggedFileExt;
use lofty::probe::Probe;
use lofty::tag::{ItemValue, Tag};
use std::collections::BTreeMap;

/// Tag values extracted from an audio file for snapshot testing.
#[derive(Debug, Serialize)]
pub struct TagSnapshot {
    pub tags: BTreeMap<String, BTreeMap<String, String>>,
    pub pictures: Vec<PictureSnapshot>,
}

/// Embedded picture metadata for snapshot testing.
#[derive(Debug, Serialize)]
pub struct PictureSnapshot {
    pub picture_type: String,
    pub mime_type: String,
    pub data_size: usize,
}

impl TagSnapshot {
    /// Read all tags and pictures from an audio file.
    pub fn from_path(path: &Path) -> Self {
        let tagged_file = Probe::open(path)
            .expect("should open file")
            .read()
            .expect("should read tagged file");
        let tags = tagged_file
            .tags()
            .iter()
            .map(|tag| {
                let tag_type = format!("{:?}", tag.tag_type());
                let items = Self::extract_text_items(tag);
                (tag_type, items)
            })
            .collect();
        let pictures: Vec<PictureSnapshot> = tagged_file
            .tags()
            .iter()
            .flat_map(Tag::pictures)
            .map(|pic| PictureSnapshot {
                picture_type: format!("{:?}", pic.pic_type()),
                mime_type: pic
                    .mime_type()
                    .map_or_else(|| "unknown".to_owned(), |m| format!("{m:?}")),
                data_size: pic.data().len(),
            })
            .collect();
        Self { tags, pictures }
    }

    fn extract_text_items(tag: &Tag) -> BTreeMap<String, String> {
        let mut items = BTreeMap::new();
        for item in tag.items() {
            let key = format!("{:?}", item.key());
            let value = match item.value() {
                ItemValue::Text(text) | ItemValue::Locator(text) => text.clone(),
                ItemValue::Binary(data) => {
                    format!("<binary {} bytes>", data.len())
                }
            };
            items.entry(key).or_insert(value);
        }
        items
    }
}
