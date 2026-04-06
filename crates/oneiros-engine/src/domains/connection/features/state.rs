use crate::*;

pub struct ConnectionState;

impl ConnectionState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        match event {
            Events::Connection(ConnectionEvents::ConnectionCreated(connection)) => {
                canon
                    .connections
                    .insert(connection.id.to_string(), connection.clone());
            }
            Events::Connection(ConnectionEvents::ConnectionRemoved(removed)) => {
                canon.connections.remove(&removed.id.to_string());
            }
            _ => {}
        }

        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
