use crate::*;

pub struct BrainState;

impl BrainState {
    pub fn reduce(mut canon: SystemCanon, event: &Events) -> SystemCanon {
        if let Events::Brain(BrainEvents::BrainCreated(brain)) = event {
            canon.brains.set(brain);
        }

        canon
    }

    pub fn reducer() -> Reducer<SystemCanon> {
        Reducer::new(Self::reduce)
    }
}
