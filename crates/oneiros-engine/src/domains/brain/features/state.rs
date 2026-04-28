use crate::*;

pub struct BrainState;

impl BrainState {
    pub fn reduce(mut canon: SystemCanon, event: &Events) -> SystemCanon {
        if let Events::Brain(brain_event) = event
            && let Some(brain) = brain_event.maybe_brain()
        {
            canon.brains.set(&brain);
        }

        canon
    }

    pub fn reducer() -> Reducer<SystemCanon> {
        Reducer::new(Self::reduce)
    }
}
