use crate::*;

pub(crate) struct ConnectionState;

impl ConnectionState {
    pub(crate) fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Connection(connection_event) = event {
            match connection_event {
                ConnectionEvents::ConnectionCreated(created) => {
                    if let Ok(current) = created.current() {
                        canon.connections.set(&current.connection);
                    }
                }
                ConnectionEvents::ConnectionRemoved(removed) => {
                    if let Ok(current) = removed.current() {
                        canon.connections.remove(current.id);
                    }
                }
            };
        }

        canon
    }

    pub(crate) fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
