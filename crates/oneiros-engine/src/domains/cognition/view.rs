use crate::*;

pub struct CognitionView<'a> {
    response: CognitionResponse,
    request: &'a CognitionRequest,
}

impl<'a> CognitionView<'a> {
    pub fn new(response: CognitionResponse, request: &'a CognitionRequest) -> Self {
        Self { response, request }
    }

    pub fn mcp(&self) -> McpResponse {
        match &self.response {
            CognitionResponse::CognitionAdded(wrapped) => {
                let ref_token = RefToken::from(Ref::cognition(wrapped.data.id));
                McpResponse::new(format!(
                    "Thought recorded ({}).\n\n**texture:** {}\n**ref:** {}",
                    wrapped.data.texture, wrapped.data.texture, ref_token
                ))
                .hint_set(HintSet::cognition_added(
                    CognitionAddedHints::builder()
                        .agent(AgentName::new(wrapped.data.agent_id.to_string()))
                        .ref_token(ref_token)
                        .build(),
                ))
            }
            CognitionResponse::CognitionDetails(wrapped) => {
                let ref_token = RefToken::from(Ref::cognition(wrapped.data.id));
                McpResponse::new(format!(
                    "# Cognition\n\n**texture:** {}\n**agent:** {}\n**created:** {}\n\n{}\n",
                    wrapped.data.texture,
                    wrapped.data.agent_id,
                    wrapped.data.created_at,
                    wrapped.data.content
                ))
                .hint(Hint::suggest(
                    format!("create-connection <nature> {ref_token} <target>"),
                    "Connect to something related",
                ))
                .hint(Hint::suggest("search-query", "Search for related entities"))
            }
            CognitionResponse::Cognitions(listed) => {
                let title = match self.request {
                    CognitionRequest::ListCognitions(listing) => match &listing.agent {
                        Some(agent) => format!("# Cognitions — {agent}\n\n"),
                        None => "# Cognitions\n\n".to_string(),
                    },
                    _ => "# Cognitions\n\n".to_string(),
                };
                let mut md = format!("{title}{} of {} total\n\n", listed.len(), listed.total);
                for wrapped in &listed.items {
                    md.push_str(&format!(
                        "### {} — {}\n{}\n\n",
                        wrapped.data.texture, wrapped.data.created_at, wrapped.data.content
                    ));
                }
                let mut response =
                    McpResponse::new(md).hint(Hint::suggest("add-cognition", "Record a thought"));
                if let CognitionRequest::ListCognitions(listing) = self.request
                    && let Some(agent) = &listing.agent
                {
                    response = response.hint(Hint::inspect(
                        ResourcePath::AgentMemories(agent.clone()).uri(),
                        "Browse memories",
                    ));
                }
                response
            }
            CognitionResponse::NoCognitions => McpResponse::new("No cognitions yet.")
                .hint(Hint::suggest("add-cognition", "Record a thought")),
        }
    }

    pub fn render(self) -> Rendered<CognitionResponse> {
        match (self.response, self.request) {
            (
                CognitionResponse::CognitionAdded(wrapped),
                CognitionRequest::AddCognition(addition),
            ) => {
                let subject = wrapped
                    .meta()
                    .ref_token()
                    .map(|ref_token| {
                        format!(
                            "{} Cognition recorded: {}",
                            "✓".success(),
                            ref_token.muted()
                        )
                    })
                    .unwrap_or_default();

                let hints = match wrapped.meta().ref_token() {
                    Some(ref_token) => HintSet::cognition_added(
                        CognitionAddedHints::builder()
                            .agent(addition.agent.clone())
                            .ref_token(ref_token)
                            .build(),
                    ),
                    None => HintSet::None,
                };

                Rendered::new(
                    CognitionResponse::CognitionAdded(wrapped),
                    subject,
                    String::new(),
                )
                .with_hints(hints)
            }
            (CognitionResponse::CognitionDetails(wrapped), _) => {
                let prompt = Detail::new(wrapped.data.texture.to_string())
                    .field("content:", wrapped.data.content.to_string())
                    .to_string();

                let hints = match wrapped.meta().ref_token() {
                    Some(ref_token) => {
                        HintSet::mutation(MutationHints::builder().ref_token(ref_token).build())
                    }
                    None => HintSet::None,
                };

                Rendered::new(
                    CognitionResponse::CognitionDetails(wrapped),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
            (CognitionResponse::Cognitions(listed), _) => {
                let mut table = Table::new(vec![
                    Column::key("texture", "Texture"),
                    Column::key("content", "Content").max(60),
                    Column::key("ref_token", "Ref"),
                ]);

                for wrapped in &listed.items {
                    let ref_token = wrapped
                        .meta()
                        .ref_token()
                        .map(|t| t.to_string())
                        .unwrap_or_default();
                    table.push_row(vec![
                        wrapped.data.texture.to_string(),
                        wrapped.data.content.to_string(),
                        ref_token,
                    ]);
                }

                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                );

                Rendered::new(CognitionResponse::Cognitions(listed), prompt, String::new())
            }
            (CognitionResponse::NoCognitions, _) => Rendered::new(
                CognitionResponse::NoCognitions,
                format!("{}", "No cognitions.".muted()),
                String::new(),
            ),
            (response, _) => Rendered::new(response, String::new(), String::new()),
        }
    }
}
