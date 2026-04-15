//! Connection view — presentation authority for the connection domain.

use crate::*;

pub struct ConnectionView {
    response: ConnectionResponse,
}

impl ConnectionView {
    pub fn new(response: ConnectionResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<ConnectionResponse> {
        match self.response {
            ConnectionResponse::ConnectionCreated(wrapped) => {
                let subject = wrapped
                    .meta()
                    .ref_token()
                    .map(|ref_token| {
                        format!(
                            "{} Connection recorded: {}",
                            "✓".success(),
                            ref_token.muted()
                        )
                    })
                    .unwrap_or_default();
                let hints = match wrapped.meta().ref_token() {
                    Some(ref_token) => {
                        HintSet::mutation(MutationHints::builder().ref_token(ref_token).build())
                    }
                    None => HintSet::None,
                };
                Rendered::new(
                    ConnectionResponse::ConnectionCreated(wrapped),
                    subject,
                    String::new(),
                )
                .with_hints(hints)
            }
            ConnectionResponse::ConnectionDetails(wrapped) => {
                let prompt = Detail::new(wrapped.data.nature.to_string())
                    .field("from:", wrapped.data.from_ref.to_string())
                    .field("to:", wrapped.data.to_ref.to_string())
                    .to_string();
                let hints = match wrapped.meta().ref_token() {
                    Some(ref_token) => {
                        HintSet::mutation(MutationHints::builder().ref_token(ref_token).build())
                    }
                    None => HintSet::None,
                };
                Rendered::new(
                    ConnectionResponse::ConnectionDetails(wrapped),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
            ConnectionResponse::Connections(listed) => {
                let mut table = Table::new(vec![
                    Column::key("nature", "Nature"),
                    Column::key("from_ref", "From"),
                    Column::key("to_ref", "To"),
                    Column::key("ref_token", "Ref"),
                ]);
                for wrapped in &listed.items {
                    let ref_token = wrapped
                        .meta()
                        .ref_token()
                        .map(|t| t.to_string())
                        .unwrap_or_default();
                    table.push_row(vec![
                        wrapped.data.nature.to_string(),
                        wrapped.data.from_ref.to_string(),
                        wrapped.data.to_ref.to_string(),
                        ref_token,
                    ]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                );
                Rendered::new(
                    ConnectionResponse::Connections(listed),
                    prompt,
                    String::new(),
                )
            }
            ConnectionResponse::NoConnections => Rendered::new(
                ConnectionResponse::NoConnections,
                format!("{}", "No connections.".muted()),
                String::new(),
            ),
            ConnectionResponse::ConnectionRemoved(id) => {
                let prompt = Confirmation::new("Connection", id.to_string(), "removed").to_string();
                Rendered::new(
                    ConnectionResponse::ConnectionRemoved(id),
                    prompt,
                    String::new(),
                )
            }
        }
    }
}
