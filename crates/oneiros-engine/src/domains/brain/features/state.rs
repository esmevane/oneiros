use crate::*;

pub(crate) struct BrainState;

impl BrainState {
    pub(crate) fn reduce(mut canon: SystemCanon, event: &Events) -> SystemCanon {
        if let Events::Brain(BrainEvents::BrainCreated(brain)) = event {
            canon.brains.set(brain);
        }

        canon
    }

    pub(crate) fn reducer() -> Reducer<SystemCanon> {
        Reducer::new(Self::reduce)
    }
}
