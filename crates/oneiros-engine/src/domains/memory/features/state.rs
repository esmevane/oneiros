use crate::*;

pub struct MemoryState;

impl MemoryState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Memory(MemoryEvents::MemoryAdded(memory)) = event {
            canon.memories.set(memory);
        }

        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
