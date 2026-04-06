use crate::*;

pub struct StorageState;

impl StorageState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        match event {
            Events::Storage(StorageEvents::StorageSet(entry)) => {
                canon.storage.insert(entry.key.to_string(), entry.clone());
            }
            Events::Storage(StorageEvents::StorageRemoved(removed)) => {
                canon.storage.remove(&removed.key.to_string());
            }
            _ => {}
        }

        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
