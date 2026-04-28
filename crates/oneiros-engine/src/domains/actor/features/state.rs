use crate::*;

pub struct ActorState;

impl ActorState {
    pub fn reduce(mut canon: SystemCanon, event: &Events) -> SystemCanon {
        if let Events::Actor(actor_event) = event
            && let Some(actor) = actor_event.maybe_actor()
        {
            canon.actors.set(&actor);
        }

        canon
    }

    pub fn reducer() -> Reducer<SystemCanon> {
        Reducer::new(Self::reduce)
    }
}
