use crate::*;

pub struct ConnectionState;

impl ConnectionState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
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

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
