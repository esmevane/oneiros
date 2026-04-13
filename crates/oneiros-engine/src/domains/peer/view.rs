//! Peer view — presentation authority for the peer domain.

use crate::*;

pub(crate) struct PeerView;

impl PeerView {
    /// Table of peers with standard columns.
    pub(crate) fn table(peers: &Listed<Response<Peer>>) -> Table {
        let mut table = Table::new(vec![
            Column::key("name", "Name"),
            Column::key("key", "Key"),
            Column::key("id", "ID"),
        ]);

        for wrapped in &peers.items {
            let peer = &wrapped.data;
            let key_display = peer.key.to_string();
            let short_key: String = key_display.chars().take(12).collect();
            table.push_row(vec![
                peer.name.to_string(),
                format!("{short_key}…"),
                peer.id.to_string(),
            ]);
        }

        table
    }

    /// Detail view for a single peer.
    pub(crate) fn detail(peer: &Peer) -> Detail {
        Detail::new(peer.name.to_string())
            .field("id:", peer.id.to_string())
            .field("key:", peer.key.to_string())
            .field("address:", peer.address.to_string())
            .field("created_at:", peer.created_at.as_string())
    }

    /// Confirmation for a mutation.
    pub(crate) fn confirmed(verb: &str, name: &PeerName) -> Confirmation {
        Confirmation::new("Peer", name.to_string(), verb)
    }
}
