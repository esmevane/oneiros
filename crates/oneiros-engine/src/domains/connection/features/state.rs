use crate::*;

pub(crate) struct ConnectionState;

impl ConnectionState {
    pub(crate) fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Connection(connection_event) = event {
            match connection_event {
                ConnectionEvents::ConnectionCreated(connection) => {
                    canon.connections.set(connection);
                }
                ConnectionEvents::ConnectionRemoved(removed) => {
                    canon.connections.remove(removed.id);
                }
            };
        }

        canon
    }

    pub(crate) fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
