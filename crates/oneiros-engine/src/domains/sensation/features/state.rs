use crate::*;

pub(crate) struct SensationState;

impl SensationState {
    pub(crate) fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
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

    pub(crate) fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
