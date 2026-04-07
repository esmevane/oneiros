use crate::*;

pub struct StorageState;

impl StorageState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Storage(storage_event) = event {
            match storage_event {
                StorageEvents::StorageSet(entry) => {
                    canon.storage.set(entry);
                }
                StorageEvents::StorageRemoved(removed) => {
                    canon.storage.remove(&removed.key);
                }
            };
        }

        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
