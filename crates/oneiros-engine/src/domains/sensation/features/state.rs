use crate::*;

pub struct SensationState;

impl SensationState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Sensation(sensation_event) = event {
            match sensation_event {
                SensationEvents::SensationSet(setting) => {
                    if let Ok(current) = setting.current() {
                        canon.sensations.set(&current.sensation);
                    }
                }
                SensationEvents::SensationRemoved(removal) => {
                    if let Ok(current) = removal.current() {
                        canon.sensations.remove(&current.name);
                    }
                }
            };
        }

        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
