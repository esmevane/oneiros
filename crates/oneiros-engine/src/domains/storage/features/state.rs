use crate::*;

pub struct StorageState;

impl StorageState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Storage(storage_event) = event {
            match storage_event {
                StorageEvents::StorageSet(set) => {
                    if let Ok(current) = set.current() {
                        canon.storage.set(&current.entry);
                    }
                }
                StorageEvents::StorageRemoved(removed) => {
                    if let Ok(current) = removed.current() {
                        canon.storage.remove(&current.key);
                    }
                }
            };
        }

        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
