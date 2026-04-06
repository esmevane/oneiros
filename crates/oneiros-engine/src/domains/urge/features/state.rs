use crate::*;

pub struct UrgeState;

impl UrgeState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        match event {
            Events::Urge(UrgeEvents::UrgeSet(urge)) => {
                canon.urges.insert(urge.name.to_string(), urge.clone());
            }
            Events::Urge(UrgeEvents::UrgeRemoved(removed)) => {
                canon.urges.remove(&removed.name.to_string());
            }
            _ => {}
        }

        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
