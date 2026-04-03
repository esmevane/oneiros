use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum ExperienceCommands {
    Create(CreateExperience),
    Show(GetExperience),
    List(ListExperiences),
    Update {
        id: String,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        sensation: Option<String>,
    },
}

impl ExperienceCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, ExperienceError> {
        let client = context.client();
        let experience_client = ExperienceClient::new(&client);

        let response = match self {
            ExperienceCommands::Create(creation) => experience_client.create(creation).await?,
            ExperienceCommands::Show(get) => experience_client.get(get).await?,
            ExperienceCommands::List(listing) => experience_client.list(listing).await?,
            ExperienceCommands::Update {
                id,
                description,
                sensation,
            } => {
                let id: ExperienceId = id.parse()?;
                let mut result: Option<ExperienceResponse> = None;

                if let Some(desc) = description {
                    result = Some(
                        experience_client
                            .update_description(
                                &UpdateExperienceDescription::builder()
                                    .id(id)
                                    .description(Description::new(desc))
                                    .build(),
                            )
                            .await?,
                    );
                }

                if let Some(sens) = sensation {
                    result = Some(
                        experience_client
                            .update_sensation(
                                &UpdateExperienceSensation::builder()
                                    .id(id)
                                    .sensation(SensationName::new(sens))
                                    .build(),
                            )
                            .await?,
                    );
                }

                result.ok_or_else(|| {
                    ExperienceError::InvalidRequest(
                        "update requires --description or --sensation".into(),
                    )
                })?
            }
        };

        let prompt = match &response {
            ExperienceResponse::ExperienceCreated(wrapped) => wrapped
                .meta()
                .ref_token()
                .map(|ref_token| format!("Experience recorded: {ref_token}"))
                .unwrap_or_default(),
            ExperienceResponse::ExperienceDetails(wrapped) => {
                format!("[{}] {}", wrapped.data.sensation, wrapped.data.description)
            }
            ExperienceResponse::Experiences(listed) => {
                let mut out = format!("{} found of {} total.\n\n", listed.len(), listed.total);
                for wrapped in &listed.items {
                    let ref_token = &wrapped
                        .meta()
                        .ref_token()
                        .map(|ref_token| ref_token.to_string())
                        .unwrap_or_default();
                    out.push_str(&format!(
                        "  [{}] {}\n    {}\n\n",
                        wrapped.data.sensation, wrapped.data.description, ref_token
                    ));
                }
                out
            }
            ExperienceResponse::NoExperiences => "No experiences.".to_string(),
            ExperienceResponse::ExperienceUpdated(wrapped) => wrapped
                .meta()
                .ref_token()
                .map(|ref_token| format!("Experience updated: {ref_token}"))
                .unwrap_or_default(),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
