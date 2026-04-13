use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum ExperienceCommands {
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
    pub(crate) async fn execute(
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
            ExperienceResponse::ExperienceCreated(wrapped) => ExperienceView::recorded(wrapped),
            ExperienceResponse::ExperienceDetails(wrapped) => {
                ExperienceView::detail(&wrapped.data).to_string()
            }
            ExperienceResponse::Experiences(listed) => {
                let table = ExperienceView::table(listed);
                format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                )
            }
            ExperienceResponse::NoExperiences => format!("{}", "No experiences.".muted()),
            ExperienceResponse::ExperienceUpdated(wrapped) => ExperienceView::updated(wrapped),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
