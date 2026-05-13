use crate::*;

pub(crate) struct NatureState;

impl NatureState {
    pub(crate) fn reduce(mut canon: ProjectCanon, event: &Events) -> ProjectCanon {
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

    pub(crate) fn reducer() -> Reducer<ProjectCanon> {
        Reducer::new(Self::reduce)
    }
}
