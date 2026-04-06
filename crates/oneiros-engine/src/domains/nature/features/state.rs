use crate::*;

pub struct NatureState;

impl NatureState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        match event {
            Events::Nature(NatureEvents::NatureSet(nature)) => {
                canon
                    .natures
                    .insert(nature.name.to_string(), nature.clone());
            }
            Events::Nature(NatureEvents::NatureRemoved(removed)) => {
                canon.natures.remove(&removed.name.to_string());
            }
            _ => {}
        }

        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
