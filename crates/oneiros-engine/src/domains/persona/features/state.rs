use crate::*;

pub struct PersonaState;

impl PersonaState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        match event {
            Events::Persona(PersonaEvents::PersonaSet(persona)) => {
                canon
                    .personas
                    .insert(persona.name.to_string(), persona.clone());
            }
            Events::Persona(PersonaEvents::PersonaRemoved(removed)) => {
                canon.personas.remove(&removed.name.to_string());
            }
            _ => {}
        }

        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
