use crate::*;

pub struct NatureState;

impl NatureState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Nature(nature_event) = event {
            match nature_event {
                NatureEvents::NatureSet(setting) => {
                    if let Ok(current) = setting.current() {
                        canon.natures.set(&current.nature);
                    }
                }
                NatureEvents::NatureRemoved(removal) => {
                    if let Ok(current) = removal.current() {
                        canon.natures.remove(&current.name);
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
