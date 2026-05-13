use crate::*;

pub(crate) struct ActorState;

impl ActorState {
    pub(crate) fn reduce(mut canon: HostCanon, event: &Events) -> HostCanon {
        if let Events::Actor(actor_event) = event
            && let Some(actor) = actor_event.maybe_actor()
        {
            canon.actors.set(&actor);
        }

        canon
    }

    pub(crate) fn reducer() -> Reducer<HostCanon> {
        Reducer::new(Self::reduce)
    }
}
