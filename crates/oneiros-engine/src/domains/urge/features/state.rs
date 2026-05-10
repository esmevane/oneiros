use crate::*;

pub(crate) struct UrgeState;

impl UrgeState {
    pub(crate) fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Urge(urge_event) = event {
            match urge_event {
                UrgeEvents::UrgeSet(setting) => {
                    if let Ok(current) = setting.current() {
                        canon.urges.set(&current.urge);
                    }
                }
                UrgeEvents::UrgeRemoved(removal) => {
                    if let Ok(current) = removal.current() {
                        canon.urges.remove(&current.name);
                    }
                }
            };
        }

        canon
    }

    pub(crate) fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
