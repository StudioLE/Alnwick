use crate::prelude::*;
use serde::de::DeserializeOwned;
use web_sys::wasm_bindgen::JsValue;
use web_sys::{window, Storage};
use LocalStorageError::*;

pub struct LocalStorage;

impl LocalStorage {
    pub fn get<T: DeserializeOwned>(key: &str) -> Result<T, LocalStorageError> {
        let serialized = get_local_storage()?
            .get_item(key)
            .map_err(Js)?
            .ok_or(NoData)?;
        serde_json::from_str(&serialized).map_err(Deserialization)
    }

    pub fn set<T: Serialize + ?Sized>(key: &str, value: &T) -> Result<(), LocalStorageError> {
        let serialized = serde_json::to_string(value).map_err(Serialization)?;
        get_local_storage()?.set_item(key, &serialized).map_err(Js)
    }
}

fn get_local_storage() -> Result<Storage, LocalStorageError> {
    window()
        .ok_or(NoWindow)?
        .local_storage()
        .map_err(Js)?
        .ok_or(NoStorage)
}

#[allow(dead_code, clippy::absolute_paths)]
#[derive(Debug)]
pub enum LocalStorageError {
    NoWindow,
    Js(JsValue),
    NoStorage,
    Serialization(serde_json::Error),
    FlushWriter,
    ToUtf8(std::string::FromUtf8Error),
    NoData,
    Deserialization(serde_json::Error),
}
