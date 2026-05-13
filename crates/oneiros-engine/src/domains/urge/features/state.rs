use crate::*;

pub(crate) struct UrgeState;

impl UrgeState {
    pub(crate) fn reduce(mut canon: ProjectCanon, event: &Events) -> ProjectCanon {
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

    pub(crate) fn reducer() -> Reducer<ProjectCanon> {
        Reducer::new(Self::reduce)
    }
}
