//! Connection view — presentation authority for the connection domain.

use crate::*;

pub struct ConnectionView<'a> {
    response: ConnectionResponse,
    request: &'a ConnectionRequest,
}

impl<'a> ConnectionView<'a> {
    pub fn new(response: ConnectionResponse, request: &'a ConnectionRequest) -> Self {
        Self { response, request }
    }

    pub fn mcp(&self) -> McpResponse {
        match &self.response {
            ConnectionResponse::ConnectionCreated(ConnectionCreatedResponse::V1(created)) => {
                let ref_token = RefToken::from(Ref::connection(created.connection.id));
                McpResponse::new(format!(
                    "Connection created.\n\n**nature:** {}\n**from:** {}\n**to:** {}\n**ref:** {}",
                    created.connection.nature,
                    created.connection.from_ref,
                    created.connection.to_ref,
                    ref_token
                ))
                .hint(Hint::suggest("search-query", "Search for related entities"))
            }
            ConnectionResponse::ConnectionDetails(ConnectionDetailsResponse::V1(details)) => {
                McpResponse::new(format!(
                    "# Connection\n\n**nature:** {}\n**from:** {}\n**to:** {}\n**created:** {}\n",
                    details.connection.nature,
                    details.connection.from_ref,
                    details.connection.to_ref,
                    details.connection.created_at
                ))
                .hint(Hint::suggest("search-query", "Search for related entities"))
            }
            ConnectionResponse::Connections(ConnectionsResponse::V1(listed)) => {
                let title = match self.request {
                    ConnectionRequest::ListConnections(ListConnections::V1(listing)) => {
                        match &listing.entity {
                            Some(entity) => format!("# Connections — {entity}\n\n"),
                            None => "# Connections\n\n".to_string(),
                        }
                    }
                    _ => "# Connections\n\n".to_string(),
                };
                let mut md = format!(
                    "{title}{} of {} total\n\n",
                    listed.items.len(),
                    listed.total
                );
                for item in &listed.items {
                    md.push_str(&format!(
                        "- **{}** {} → {}\n",
                        item.nature, item.from_ref, item.to_ref
                    ));
                }
                McpResponse::new(md)
                    .hint(Hint::suggest(
                        "create-connection",
                        "Draw a line between related things",
                    ))
                    .hint(Hint::suggest("search-query", "Search for related entities"))
            }
            ConnectionResponse::NoConnections => McpResponse::new("No connections yet."),
            ConnectionResponse::ConnectionRemoved(ConnectionRemovedResponse::V1(removed)) => {
                McpResponse::new(format!("Connection removed: {}", removed.id))
            }
        }
    }

    pub fn render(self) -> Rendered<ConnectionResponse> {
        match self.response {
            ConnectionResponse::ConnectionCreated(ConnectionCreatedResponse::V1(created)) => {
                let ref_token = RefToken::from(Ref::connection(created.connection.id));
                let subject = format!(
                    "{} Connection recorded: {}",
                    "✓".success(),
                    ref_token.clone().muted()
                );
                let hints =
                    HintSet::mutation(MutationHints::builder().ref_token(ref_token).build());
                Rendered::new(
                    ConnectionResponse::ConnectionCreated(ConnectionCreatedResponse::V1(created)),
                    subject,
                    String::new(),
                )
                .with_hints(hints)
            }
            ConnectionResponse::ConnectionDetails(ConnectionDetailsResponse::V1(details)) => {
                let prompt = Detail::new(details.connection.nature.to_string())
                    .field("from:", details.connection.from_ref.to_string())
                    .field("to:", details.connection.to_ref.to_string())
                    .to_string();
                let ref_token = RefToken::from(Ref::connection(details.connection.id));
                let hints =
                    HintSet::mutation(MutationHints::builder().ref_token(ref_token).build());
                Rendered::new(
                    ConnectionResponse::ConnectionDetails(ConnectionDetailsResponse::V1(details)),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
            ConnectionResponse::Connections(ConnectionsResponse::V1(listed)) => {
                let mut table = Table::new(vec![
                    Column::key("nature", "Nature"),
                    Column::key("from_ref", "From"),
                    Column::key("to_ref", "To"),
                    Column::key("ref_token", "Ref"),
                ]);
                for item in &listed.items {
                    let ref_token = RefToken::from(Ref::connection(item.id));
                    table.push_row(vec![
                        item.nature.to_string(),
                        item.from_ref.to_string(),
                        item.to_ref.to_string(),
                        ref_token.to_string(),
                    ]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.items.len(), listed.total).muted(),
                );
                Rendered::new(
                    ConnectionResponse::Connections(ConnectionsResponse::V1(listed)),
                    prompt,
                    String::new(),
                )
            }
            ConnectionResponse::NoConnections => Rendered::new(
                ConnectionResponse::NoConnections,
                format!("{}", "No connections.".muted()),
                String::new(),
            ),
            ConnectionResponse::ConnectionRemoved(ConnectionRemovedResponse::V1(removed)) => {
                let prompt =
                    Confirmation::new("Connection", removed.id.to_string(), "removed").to_string();
                Rendered::new(
                    ConnectionResponse::ConnectionRemoved(ConnectionRemovedResponse::V1(removed)),
                    prompt,
                    String::new(),
                )
            }
        }
    }
}
