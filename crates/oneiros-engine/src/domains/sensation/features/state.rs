use crate::*;

pub struct SensationState;

impl SensationState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        match event {
            Events::Sensation(SensationEvents::SensationSet(sensation)) => {
                canon
                    .sensations
                    .insert(sensation.name.to_string(), sensation.clone());
            }
            Events::Sensation(SensationEvents::SensationRemoved(removed)) => {
                canon.sensations.remove(&removed.name.to_string());
            }
            _ => {}
        }

        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
