use crate::*;

pub struct ActorState;

impl ActorState {
    pub fn reduce(mut canon: SystemCanon, event: &Events) -> SystemCanon {
        if let Events::Actor(ActorEvents::ActorCreated(actor)) = event {
            canon.actors.insert(actor.id.to_string(), actor.clone());
        }

        canon
    }

    pub fn reducer() -> Reducer<SystemCanon> {
        Reducer::new(Self::reduce)
    }
}
