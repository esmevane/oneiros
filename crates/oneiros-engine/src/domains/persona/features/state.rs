use crate::*;

pub struct PersonaState;

impl PersonaState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Persona(persona_event) = event {
            match persona_event {
                PersonaEvents::PersonaSet(persona) => {
                    canon.personas.set(persona);
                }
                PersonaEvents::PersonaRemoved(removed) => {
                    canon.personas.remove(removed.name());
                }
            };
        }

        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
