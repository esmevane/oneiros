use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum ExperienceCommands {
    Create(CreateExperience),
    Show(GetExperience),
    List(ListExperiences),
    /// Update an experience's description and/or sensation. The
    /// command dispatches to one of the protocol-level update
    /// requests based on which fields are supplied.
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
        context: &ProjectLog,
    ) -> Result<Rendered<Responses>, ExperienceError> {
        let client = context.client();
        let experience_client = ExperienceClient::new(&client);

        let (response, request) = match self {
            Self::Create(creation) => (
                experience_client.create(creation).await?,
                ExperienceRequest::CreateExperience(creation.clone()),
            ),
            Self::Show(lookup) => (
                experience_client.get(lookup).await?,
                ExperienceRequest::GetExperience(lookup.clone()),
            ),
            Self::List(listing) => (
                experience_client.list(listing).await?,
                ExperienceRequest::ListExperiences(listing.clone()),
            ),
            Self::Update {
                id,
                description,
                sensation,
            } => {
                let id: ExperienceId = id.parse()?;
                let mut result: Option<(ExperienceResponse, ExperienceRequest)> = None;

                if let Some(desc) = description {
                    let update: UpdateExperienceDescription =
                        UpdateExperienceDescription::builder_v1()
                            .id(id)
                            .description(Description::new(desc))
                            .build()
                            .into();
                    let response = experience_client.update_description(&update).await?;
                    result = Some((
                        response,
                        ExperienceRequest::UpdateExperienceDescription(update),
                    ));
                }

                if let Some(sens) = sensation {
                    let update: UpdateExperienceSensation = UpdateExperienceSensation::builder_v1()
                        .id(id)
                        .sensation(SensationName::new(sens))
                        .build()
                        .into();
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
