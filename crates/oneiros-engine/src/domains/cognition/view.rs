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
            CognitionResponse::CognitionAdded(CognitionAddedResponse::V1(added)) => {
                let ref_token = RefToken::from(Ref::cognition(added.cognition.id));
                McpResponse::new(format!(
                    "Thought recorded ({}).\n\n**texture:** {}\n**ref:** {}",
                    added.cognition.texture, added.cognition.texture, ref_token
                ))
                .hint_set(HintSet::cognition_added(
                    CognitionAddedHints::builder()
                        .agent(AgentName::new(added.cognition.agent_id.to_string()))
                        .ref_token(ref_token)
                        .build(),
                ))
            }
            CognitionResponse::CognitionDetails(CognitionDetailsResponse::V1(details)) => {
                let ref_token = RefToken::from(Ref::cognition(details.cognition.id));
                McpResponse::new(format!(
                    "# Cognition\n\n**texture:** {}\n**agent:** {}\n**created:** {}\n\n{}\n",
                    details.cognition.texture,
                    details.cognition.agent_id,
                    details.cognition.created_at,
                    details.cognition.content
                ))
                .hint(Hint::suggest(
                    format!("create-connection <nature> {ref_token} <target>"),
                    "Connect to something related",
                ))
                .hint(Hint::suggest("search-query", "Search for related entities"))
            }
            CognitionResponse::Cognitions(CognitionsResponse::V1(listed)) => {
                let title = match self.request {
                    CognitionRequest::ListCognitions(ListCognitions::V1(listing)) => {
                        match &listing.agent {
                            Some(agent) => format!("# Cognitions — {agent}\n\n"),
                            None => "# Cognitions\n\n".to_string(),
                        }
                    }
                    _ => "# Cognitions\n\n".to_string(),
                };
                let mut md = format!(
                    "{title}{} of {} total\n\n",
                    listed.items.len(),
                    listed.total
                );
                for item in &listed.items {
                    md.push_str(&format!(
                        "### {} — {}\n{}\n\n",
                        item.texture, item.created_at, item.content
                    ));
                }
                let mut response =
                    McpResponse::new(md).hint(Hint::suggest("add-cognition", "Record a thought"));
                if let CognitionRequest::ListCognitions(ListCognitions::V1(listing)) = self.request
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
                CognitionResponse::CognitionAdded(CognitionAddedResponse::V1(added)),
                CognitionRequest::AddCognition(AddCognition::V1(addition)),
            ) => {
                let ref_token = RefToken::from(Ref::cognition(added.cognition.id));
                let subject = format!(
                    "{} Cognition recorded: {}",
                    "✓".success(),
                    ref_token.clone().muted()
                );
                let hints = HintSet::cognition_added(
                    CognitionAddedHints::builder()
                        .agent(addition.agent.clone())
                        .ref_token(ref_token)
                        .build(),
                );

                Rendered::new(
                    CognitionResponse::CognitionAdded(CognitionAddedResponse::V1(added)),
                    subject,
                    String::new(),
                )
                .with_hints(hints)
            }
            (CognitionResponse::CognitionDetails(CognitionDetailsResponse::V1(details)), _) => {
                let prompt = Detail::new(details.cognition.texture.to_string())
                    .field("content:", details.cognition.content.to_string())
                    .to_string();
                let ref_token = RefToken::from(Ref::cognition(details.cognition.id));
                let hints =
                    HintSet::mutation(MutationHints::builder().ref_token(ref_token).build());

                Rendered::new(
                    CognitionResponse::CognitionDetails(CognitionDetailsResponse::V1(details)),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
            (CognitionResponse::Cognitions(CognitionsResponse::V1(listed)), _) => {
                let mut table = Table::new(vec![
                    Column::key("texture", "Texture"),
                    Column::key("content", "Content").max(60),
                    Column::key("ref_token", "Ref"),
                ]);

                for item in &listed.items {
                    let ref_token = RefToken::from(Ref::cognition(item.id));
                    table.push_row(vec![
                        item.texture.to_string(),
                        item.content.to_string(),
                        ref_token.to_string(),
                    ]);
                }

                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.items.len(), listed.total).muted(),
                );

                Rendered::new(
                    CognitionResponse::Cognitions(CognitionsResponse::V1(listed)),
                    prompt,
                    String::new(),
                )
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
