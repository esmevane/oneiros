use crate::*;

pub struct SensationState;

impl SensationState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Sensation(sensation_event) = event {
            match sensation_event {
                SensationEvents::SensationSet(sensation) => {
                    canon.sensations.set(sensation);
                }
                SensationEvents::SensationRemoved(removed) => {
                    canon.sensations.remove(&removed.name);
                }
            };
        }

        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
