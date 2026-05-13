use crate::*;

pub(crate) struct PeerState;

impl PeerState {
    pub(crate) fn reduce(mut canon: HostCanon, event: &Events) -> HostCanon {
        if let Events::Peer(peer_event) = event {
            match peer_event {
                PeerEvents::PeerAdded(added) => {
                    if let Ok(current) = added.current() {
                        let peer = Peer::builder()
                            .id(current.id)
                            .key(current.key)
                            .address(current.address)
                            .name(current.name)
                            .created_at(current.created_at)
                            .build();
                        canon.peers.set(&peer);
                    }
                }
                PeerEvents::PeerUpdated(updated) => {
                    if let Ok(current) = updated.current() {
                        let peer = Peer::builder()
                            .id(current.id)
                            .key(current.key)
                            .address(current.address)
                            .name(current.name)
                            .created_at(current.created_at)
                            .build();
                        canon.peers.set(&peer);
                    }
                }
                PeerEvents::PeerRemoved(removed) => {
                    if let Ok(current) = removed.current() {
                        canon.peers.remove(&current.id);
                    }
                }
            };
        }

        canon
    }

    pub(crate) fn reducer() -> Reducer<HostCanon> {
        Reducer::new(Self::reduce)
    }
}
