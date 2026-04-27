use crate::*;

pub struct PeerState;

impl PeerState {
    pub fn reduce(mut canon: SystemCanon, event: &Events) -> SystemCanon {
        if let Events::Peer(peer_event) = event {
            match peer_event {
                PeerEvents::PeerAdded(peer) => {
                    canon.peers.set(peer);
                }
                PeerEvents::PeerUpdated(peer) => {
                    canon.peers.set(peer);
                }
                PeerEvents::PeerRemoved(removed) => {
                    canon.peers.remove(removed.id());
                }
            };
        }

        canon
    }

    pub fn reducer() -> Reducer<SystemCanon> {
        Reducer::new(Self::reduce)
    }
}
