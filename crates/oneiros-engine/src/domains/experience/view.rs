//! Experience view — presentation authority for the experience domain.

use crate::*;

pub struct ExperienceView<'a> {
    response: ExperienceResponse,
    request: &'a ExperienceRequest,
}

impl<'a> ExperienceView<'a> {
    pub fn new(response: ExperienceResponse, request: &'a ExperienceRequest) -> Self {
        Self { response, request }
    }

    pub fn mcp(&self) -> McpResponse {
        match &self.response {
            ExperienceResponse::ExperienceCreated(wrapped) => {
                let ref_token = RefToken::from(Ref::experience(wrapped.data.id));
                McpResponse::new(format!(
                    "Experience created ({}).\n\n**sensation:** {}\n**ref:** {}",
                    wrapped.data.sensation, wrapped.data.sensation, ref_token
                ))
                .hint_set(HintSet::mutation(
                    MutationHints::builder().ref_token(ref_token).build(),
                ))
            }
            ExperienceResponse::ExperienceDetails(wrapped) => {
                let ref_token = RefToken::from(Ref::experience(wrapped.data.id));
                McpResponse::new(format!(
                    "# Experience\n\n**sensation:** {}\n**agent:** {}\n**created:** {}\n\n{}\n",
                    wrapped.data.sensation,
                    wrapped.data.agent_id,
                    wrapped.data.created_at,
                    wrapped.data.description
                ))
                .hint(Hint::suggest(
                    format!("create-connection <nature> {ref_token} <target>"),
                    "Connect to something related",
                ))
                .hint(Hint::suggest("search-query", "Search for related entities"))
            }
            ExperienceResponse::Experiences(listed) => {
                let title = match self.request {
                    ExperienceRequest::ListExperiences(listing) => match &listing.agent {
                        Some(agent) => format!("# Experiences — {agent}\n\n"),
                        None => "# Experiences\n\n".to_string(),
                    },
                    _ => "# Experiences\n\n".to_string(),
                };
                let mut md = format!("{title}{} of {} total\n\n", listed.len(), listed.total);
                for wrapped in &listed.items {
                    md.push_str(&format!(
                        "### {} — {}\n{}\n\n",
                        wrapped.data.sensation, wrapped.data.created_at, wrapped.data.description
                    ));
                }
                let mut response = McpResponse::new(md).hint(Hint::suggest(
                    "create-experience",
                    "Mark a meaningful moment",
                ));
                if let ExperienceRequest::ListExperiences(listing) = self.request
                    && let Some(agent) = &listing.agent
                {
                    response = response.hint(Hint::inspect(
                        ResourcePath::AgentConnections(agent.clone()).uri(),
                        "Browse connections",
                    ));
                }
                response
            }
            ExperienceResponse::NoExperiences => McpResponse::new("No experiences yet."),
            ExperienceResponse::ExperienceUpdated(wrapped) => {
                let ref_token = RefToken::from(Ref::experience(wrapped.data.id));
                McpResponse::new(format!(
                    "Experience updated ({}).\n\n**sensation:** {}\n**ref:** {}",
                    wrapped.data.sensation, wrapped.data.sensation, ref_token
                ))
                .hint_set(HintSet::mutation(
                    MutationHints::builder().ref_token(ref_token).build(),
                ))
            }
        }
    }

    pub fn render(self) -> Rendered<ExperienceResponse> {
        match self.response {
            ExperienceResponse::ExperienceCreated(wrapped) => {
                let subject = wrapped
                    .meta()
                    .ref_token()
                    .map(|ref_token| {
                        format!(
                            "{} Experience recorded: {}",
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
                    ExperienceResponse::ExperienceCreated(wrapped),
                    subject,
                    String::new(),
                )
                .with_hints(hints)
            }
            ExperienceResponse::ExperienceDetails(wrapped) => {
                let prompt = Detail::new(wrapped.data.sensation.to_string())
                    .field("description:", wrapped.data.description.to_string())
                    .to_string();
                let hints = match wrapped.meta().ref_token() {
                    Some(ref_token) => {
                        HintSet::mutation(MutationHints::builder().ref_token(ref_token).build())
                    }
                    None => HintSet::None,
                };
                Rendered::new(
                    ExperienceResponse::ExperienceDetails(wrapped),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
            ExperienceResponse::Experiences(listed) => {
                let mut table = Table::new(vec![
                    Column::key("sensation", "Sensation"),
                    Column::key("description", "Description").max(60),
                    Column::key("ref_token", "Ref"),
                ]);
                for wrapped in &listed.items {
                    let ref_token = wrapped
                        .meta()
                        .ref_token()
                        .map(|t| t.to_string())
                        .unwrap_or_default();
                    table.push_row(vec![
                        wrapped.data.sensation.to_string(),
                        wrapped.data.description.to_string(),
                        ref_token,
                    ]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                );
                Rendered::new(
                    ExperienceResponse::Experiences(listed),
                    prompt,
                    String::new(),
                )
            }
            ExperienceResponse::NoExperiences => Rendered::new(
                ExperienceResponse::NoExperiences,
                format!("{}", "No experiences.".muted()),
                String::new(),
            ),
            ExperienceResponse::ExperienceUpdated(wrapped) => {
                let subject = wrapped
                    .meta()
                    .ref_token()
                    .map(|ref_token| {
                        format!(
                            "{} Experience updated: {}",
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
                    ExperienceResponse::ExperienceUpdated(wrapped),
                    subject,
                    String::new(),
                )
                .with_hints(hints)
            }
        }
    }
}
