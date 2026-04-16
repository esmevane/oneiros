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

        let (response, request) = match self {
            ExperienceCommands::Create(creation) => {
                let response = experience_client.create(creation).await?;
                (
                    response,
                    ExperienceRequest::CreateExperience(creation.clone()),
                )
            }
            ExperienceCommands::Show(get) => {
                let response = experience_client.get(get).await?;
                (response, ExperienceRequest::GetExperience(get.clone()))
            }
            ExperienceCommands::List(listing) => {
                let response = experience_client.list(listing).await?;
                (
                    response,
                    ExperienceRequest::ListExperiences(listing.clone()),
                )
            }
            ExperienceCommands::Update {
                id,
                description,
                sensation,
            } => {
                let id: ExperienceId = id.parse()?;
                let mut result: Option<(ExperienceResponse, ExperienceRequest)> = None;

                if let Some(desc) = description {
                    let update = UpdateExperienceDescription::builder()
                        .id(id)
                        .description(Description::new(desc))
                        .build();
                    let response = experience_client.update_description(&update).await?;
                    result = Some((
                        response,
                        ExperienceRequest::UpdateExperienceDescription(update),
                    ));
                }

                if let Some(sens) = sensation {
                    let update = UpdateExperienceSensation::builder()
                        .id(id)
                        .sensation(SensationName::new(sens))
                        .build();
                    let response = experience_client.update_sensation(&update).await?;
                    result = Some((
                        response,
                        ExperienceRequest::UpdateExperienceSensation(update),
                    ));
                }

                result.ok_or_else(|| {
                    ExperienceError::InvalidRequest(
                        "update requires --description or --sensation".into(),
                    )
                })?
            }
        };

        Ok(ExperienceView::new(response, &request)
            .render()
            .map(Into::into))
    }
}
