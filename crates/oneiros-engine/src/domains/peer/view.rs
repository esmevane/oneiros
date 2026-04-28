use crate::*;

pub struct PeerView {
    response: PeerResponse,
}

impl PeerView {
    pub fn new(response: PeerResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<PeerResponse> {
        match self.response {
            PeerResponse::Added(PeerAddedResponse::V1(added)) => {
                let prompt = Confirmation::new("Peer", added.name.to_string(), "added").to_string();
                Rendered::new(
                    PeerResponse::Added(PeerAddedResponse::V1(added)),
                    prompt,
                    String::new(),
                )
            }
            PeerResponse::Found(PeerFoundResponse::V1(found)) => {
                let prompt = Detail::new(found.name.to_string())
                    .field("id:", found.id.to_string())
                    .field("key:", found.key.to_string())
                    .field("address:", found.address.to_string())
                    .field("created_at:", found.created_at.as_string())
                    .to_string();
                Rendered::new(
                    PeerResponse::Found(PeerFoundResponse::V1(found)),
                    prompt,
                    String::new(),
                )
            }
            PeerResponse::Listed(PeersResponse::V1(listed)) => {
                let mut table = Table::new(vec![
                    Column::key("name", "Name"),
                    Column::key("key", "Key"),
                    Column::key("id", "ID"),
                ]);
                for peer in &listed.items {
                    let key_display = peer.key.to_string();
                    let short_key: String = key_display.chars().take(12).collect();
                    table.push_row(vec![
                        peer.name.to_string(),
                        format!("{short_key}…"),
                        peer.id.to_string(),
                    ]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.items.len(), listed.total).muted(),
                );
                Rendered::new(
                    PeerResponse::Listed(PeersResponse::V1(listed)),
                    prompt,
                    String::new(),
                )
            }
            PeerResponse::Removed(PeerRemovedResponse::V1(removed)) => {
                let prompt = format!("Removed peer {}", removed.id);
                Rendered::new(
                    PeerResponse::Removed(PeerRemovedResponse::V1(removed)),
                    prompt,
                    String::new(),
                )
            }
        }
    }
}
