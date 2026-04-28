use crate::*;

pub struct PersonaState;

impl PersonaState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
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

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
