use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum ExperienceCommands {
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
    pub(crate) async fn execute(
        &self,
        config: &Config,
    ) -> Result<Rendered<Responses>, ExperienceError> {
        let client = Client::from_config(config)?;

        let (bytes, request) = match self {
            Self::Create(creation) => (
                creation.execute_request(&client).await?,
                ExperienceRequest::CreateExperience(creation.clone()),
            ),
            Self::Show(lookup) => (
                lookup.execute_request(&client).await?,
                ExperienceRequest::GetExperience(lookup.clone()),
            ),
            Self::List(listing) => (
                listing.execute_request(&client).await?,
                ExperienceRequest::ListExperiences(listing.clone()),
            ),
            Self::Update {
                id,
                description,
                sensation,
            } => {
                let id: ExperienceId = id.parse()?;
                let mut result: Option<(Vec<u8>, ExperienceRequest)> = None;

                if let Some(desc) = description {
                    let update: UpdateExperienceDescription =
                        UpdateExperienceDescription::builder_v1()
                            .id(id)
                            .description(Description::new(desc))
                            .build()
                            .into();
                    let bytes = update.execute_request(&client).await?;
                    result = Some((
                        bytes,
                        ExperienceRequest::UpdateExperienceDescription(update),
                    ));
                }

                if let Some(sens) = sensation {
                    let update: UpdateExperienceSensation = UpdateExperienceSensation::builder_v1()
                        .id(id)
                        .sensation(SensationName::new(sens))
                        .build()
                        .into();
                    let bytes = update.execute_request(&client).await?;
                    result = Some((bytes, ExperienceRequest::UpdateExperienceSensation(update)));
                }

                result.ok_or_else(|| {
                    ExperienceError::InvalidRequest(
                        "update requires --description or --sensation".into(),
                    )
                })?
            }
        };

        let response: ExperienceResponse = serde_json::from_slice(&bytes)?;
        Ok(ExperienceView::new(response, &request)
            .render()
            .map(Into::into))
    }
}
