use crate::prelude::*;

const KEY: &str = "entries";

#[derive(Clone, Copy, Debug)]
pub struct EntryState {
    pub signal: Signal<EntryCollection>,
}

impl EntryState {
    pub fn init() -> Self {
        let collection = get_from_local_storage().unwrap_or_default();
        let signal = use_signal(|| collection);
        Self { signal }
    }

    pub fn get_cloned(&self) -> EntryCollection {
        self.signal.read().clone()
    }

    pub fn is_empty(&self) -> bool {
        self.signal.read().entries.is_empty()
    }

    pub fn set(&mut self, collection: EntryCollection) {
        let entries: Vec<_> = collection.entries.values().collect();
        let _ = set_to_local_storage(&entries);
        self.signal.set(collection);
    }
}

fn get_from_local_storage() -> Option<EntryCollection> {
    let entries = LocalStorage::get(KEY)
        .handle_error(|e| warn!("Failed to get {KEY} from local storage: {e:?}"))?;
    EntryCollection::new(entries).handle_error(|e| warn!("Failed to determine range: {e:?}"))
}

fn set_to_local_storage(value: &[&Entry]) -> bool {
    LocalStorage::set(KEY, &value)
        .handle_error(|e| warn!("Failed to set {KEY} in local storage: {e:?}"))
        .is_some()
}
