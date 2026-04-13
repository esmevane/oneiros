use crate::*;

pub(crate) struct MemoryState;

impl MemoryState {
    pub(crate) fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Memory(MemoryEvents::MemoryAdded(memory)) = event {
            canon.memories.set(memory);
        }

        canon
    }

    pub(crate) fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
