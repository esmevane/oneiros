use crate::*;

pub struct UrgeState;

impl UrgeState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Urge(urge_event) = event {
            match urge_event {
                UrgeEvents::UrgeSet(urge) => {
                    canon.urges.set(urge);
                }
                UrgeEvents::UrgeRemoved(removed) => {
                    canon.urges.remove(&removed.name);
                }
            };
        }

        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
