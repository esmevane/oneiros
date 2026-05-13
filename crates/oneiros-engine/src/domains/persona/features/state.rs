use crate::*;

pub(crate) struct PersonaState;

impl PersonaState {
    pub(crate) fn reduce(mut canon: ProjectCanon, event: &Events) -> ProjectCanon {
        if let Events::Persona(persona_event) = event {
            match persona_event {
                PersonaEvents::PersonaSet(setting) => {
                    if let Ok(current) = setting.current() {
                        canon.personas.set(&current.persona);
                    }
                }
                PersonaEvents::PersonaRemoved(removal) => {
                    if let Ok(current) = removal.current() {
                        canon.personas.remove(&current.name);
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
