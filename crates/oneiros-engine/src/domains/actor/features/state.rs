use crate::*;

pub(crate) struct ActorState;

impl ActorState {
    pub(crate) fn reduce(mut canon: SystemCanon, event: &Events) -> SystemCanon {
        if let Events::Actor(actor_event) = event {
            match actor_event {
                ActorEvents::ActorCreated(actor) => canon.actors.set(actor),
            };
        }

        canon
    }

    pub(crate) fn reducer() -> Reducer<SystemCanon> {
        Reducer::new(Self::reduce)
    }
}
