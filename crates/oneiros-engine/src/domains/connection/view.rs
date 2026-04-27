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
            ConnectionResponse::ConnectionCreated(wrapped) => {
                let ref_token = RefToken::from(Ref::connection(wrapped.data.id()));
                McpResponse::new(format!(
                    "Connection created.\n\n**nature:** {}\n**from:** {}\n**to:** {}\n**ref:** {}",
                    wrapped.data.nature(),
                    wrapped.data.from_ref(),
                    wrapped.data.to_ref(),
                    ref_token
                ))
                .hint(Hint::suggest("search-query", "Search for related entities"))
            }
            ConnectionResponse::ConnectionDetails(wrapped) => McpResponse::new(format!(
                "# Connection\n\n**nature:** {}\n**from:** {}\n**to:** {}\n**created:** {}\n",
                wrapped.data.nature(),
                wrapped.data.from_ref(),
                wrapped.data.to_ref(),
                wrapped.data.created_at()
            ))
            .hint(Hint::suggest("search-query", "Search for related entities")),
            ConnectionResponse::Connections(listed) => {
                let title = match self.request {
                    ConnectionRequest::ListConnections(listing) => match &listing.entity {
                        Some(entity) => format!("# Connections — {entity}\n\n"),
                        None => "# Connections\n\n".to_string(),
                    },
                    _ => "# Connections\n\n".to_string(),
                };
                let mut md = format!("{title}{} of {} total\n\n", listed.len(), listed.total);
                for wrapped in &listed.items {
                    md.push_str(&format!(
                        "- **{}** {} → {}\n",
                        wrapped.data.nature(),
                        wrapped.data.from_ref(),
                        wrapped.data.to_ref()
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
            ConnectionResponse::ConnectionRemoved(id) => {
                McpResponse::new(format!("Connection removed: {id}"))
            }
        }
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
                let prompt = Detail::new(wrapped.data.nature().to_string())
                    .field("from:", wrapped.data.from_ref().to_string())
                    .field("to:", wrapped.data.to_ref().to_string())
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
                        wrapped.data.nature().to_string(),
                        wrapped.data.from_ref().to_string(),
                        wrapped.data.to_ref().to_string(),
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
