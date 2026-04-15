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
            PeerResponse::Added(wrapped) => {
                let prompt =
                    Confirmation::new("Peer", wrapped.data.name.to_string(), "added").to_string();
                Rendered::new(PeerResponse::Added(wrapped), prompt, String::new())
            }
            PeerResponse::Found(wrapped) => {
                let prompt = Detail::new(wrapped.data.name.to_string())
                    .field("id:", wrapped.data.id.to_string())
                    .field("key:", wrapped.data.key.to_string())
                    .field("address:", wrapped.data.address.to_string())
                    .field("created_at:", wrapped.data.created_at.as_string())
                    .to_string();
                Rendered::new(PeerResponse::Found(wrapped), prompt, String::new())
            }
            PeerResponse::Listed(listed) => {
                let mut table = Table::new(vec![
                    Column::key("name", "Name"),
                    Column::key("key", "Key"),
                    Column::key("id", "ID"),
                ]);
                for wrapped in &listed.items {
                    let peer = &wrapped.data;
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
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                );
                Rendered::new(PeerResponse::Listed(listed), prompt, String::new())
            }
            PeerResponse::Removed(id) => {
                let prompt = format!("Removed peer {id}");
                Rendered::new(PeerResponse::Removed(id), prompt, String::new())
            }
        }
    }
}
