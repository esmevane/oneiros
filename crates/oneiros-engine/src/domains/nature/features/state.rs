use crate::*;

pub(crate) struct NatureState;

impl NatureState {
    pub(crate) fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Nature(nature_event) = event {
            match nature_event {
                NatureEvents::NatureSet(nature) => {
                    canon.natures.set(nature);
                }
                NatureEvents::NatureRemoved(removed) => {
                    canon.natures.remove(&removed.name);
                }
            };
        }

        canon
    }

    pub(crate) fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
