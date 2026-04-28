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
            ExperienceResponse::ExperienceCreated(ExperienceCreatedResponse::V1(created)) => {
                let ref_token = RefToken::from(Ref::experience(created.experience.id));
                McpResponse::new(format!(
                    "Experience created ({}).\n\n**sensation:** {}\n**ref:** {}",
                    created.experience.sensation, created.experience.sensation, ref_token
                ))
                .hint_set(HintSet::mutation(
                    MutationHints::builder().ref_token(ref_token).build(),
                ))
            }
            ExperienceResponse::ExperienceDetails(ExperienceDetailsResponse::V1(details)) => {
                let ref_token = RefToken::from(Ref::experience(details.experience.id));
                McpResponse::new(format!(
                    "# Experience\n\n**sensation:** {}\n**agent:** {}\n**created:** {}\n\n{}\n",
                    details.experience.sensation,
                    details.experience.agent_id,
                    details.experience.created_at,
                    details.experience.description
                ))
                .hint(Hint::suggest(
                    format!("create-connection <nature> {ref_token} <target>"),
                    "Connect to something related",
                ))
                .hint(Hint::suggest("search-query", "Search for related entities"))
            }
            ExperienceResponse::Experiences(ExperiencesResponse::V1(listed)) => {
                let title = match self.request {
                    ExperienceRequest::ListExperiences(ListExperiences::V1(listing)) => {
                        match &listing.agent {
                            Some(agent) => format!("# Experiences — {agent}\n\n"),
                            None => "# Experiences\n\n".to_string(),
                        }
                    }
                    _ => "# Experiences\n\n".to_string(),
                };
                let mut md = format!(
                    "{title}{} of {} total\n\n",
                    listed.items.len(),
                    listed.total
                );
                for item in &listed.items {
                    md.push_str(&format!(
                        "### {} — {}\n{}\n\n",
                        item.sensation, item.created_at, item.description
                    ));
                }
                let mut response = McpResponse::new(md).hint(Hint::suggest(
                    "create-experience",
                    "Mark a meaningful moment",
                ));
                if let ExperienceRequest::ListExperiences(ListExperiences::V1(listing)) =
                    self.request
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
            ExperienceResponse::ExperienceUpdated(ExperienceUpdatedResponse::V1(updated)) => {
                let ref_token = RefToken::from(Ref::experience(updated.experience.id));
                McpResponse::new(format!(
                    "Experience updated ({}).\n\n**sensation:** {}\n**ref:** {}",
                    updated.experience.sensation, updated.experience.sensation, ref_token
                ))
                .hint_set(HintSet::mutation(
                    MutationHints::builder().ref_token(ref_token).build(),
                ))
            }
        }
    }

    pub fn render(self) -> Rendered<ExperienceResponse> {
        match self.response {
            ExperienceResponse::ExperienceCreated(ExperienceCreatedResponse::V1(created)) => {
                let ref_token = RefToken::from(Ref::experience(created.experience.id));
                let subject = format!(
                    "{} Experience recorded: {}",
                    "✓".success(),
                    ref_token.clone().muted()
                );
                let hints =
                    HintSet::mutation(MutationHints::builder().ref_token(ref_token).build());
                Rendered::new(
                    ExperienceResponse::ExperienceCreated(ExperienceCreatedResponse::V1(created)),
                    subject,
                    String::new(),
                )
                .with_hints(hints)
            }
            ExperienceResponse::ExperienceDetails(ExperienceDetailsResponse::V1(details)) => {
                let prompt = Detail::new(details.experience.sensation.to_string())
                    .field("description:", details.experience.description.to_string())
                    .to_string();
                let ref_token = RefToken::from(Ref::experience(details.experience.id));
                let hints =
                    HintSet::mutation(MutationHints::builder().ref_token(ref_token).build());
                Rendered::new(
                    ExperienceResponse::ExperienceDetails(ExperienceDetailsResponse::V1(details)),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
            ExperienceResponse::Experiences(ExperiencesResponse::V1(listed)) => {
                let mut table = Table::new(vec![
                    Column::key("sensation", "Sensation"),
                    Column::key("description", "Description").max(60),
                    Column::key("ref_token", "Ref"),
                ]);
                for item in &listed.items {
                    let ref_token = RefToken::from(Ref::experience(item.id));
                    table.push_row(vec![
                        item.sensation.to_string(),
                        item.description.to_string(),
                        ref_token.to_string(),
                    ]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.items.len(), listed.total).muted(),
                );
                Rendered::new(
                    ExperienceResponse::Experiences(ExperiencesResponse::V1(listed)),
                    prompt,
                    String::new(),
                )
            }
            ExperienceResponse::NoExperiences => Rendered::new(
                ExperienceResponse::NoExperiences,
                format!("{}", "No experiences.".muted()),
                String::new(),
            ),
            ExperienceResponse::ExperienceUpdated(ExperienceUpdatedResponse::V1(updated)) => {
                let ref_token = RefToken::from(Ref::experience(updated.experience.id));
                let subject = format!(
                    "{} Experience updated: {}",
                    "✓".success(),
                    ref_token.clone().muted()
                );
                let hints =
                    HintSet::mutation(MutationHints::builder().ref_token(ref_token).build());
                Rendered::new(
                    ExperienceResponse::ExperienceUpdated(ExperienceUpdatedResponse::V1(updated)),
                    subject,
                    String::new(),
                )
                .with_hints(hints)
            }
        }
    }
}
