use clap::Args;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum InitProjectOutcomes {
    #[outcome(message("Brain '{0}' created."))]
    BrainCreated(BrainName),
    #[outcome(message("Brain '{0}' already exists."))]
    BrainAlreadyExists(BrainName),
}

#[derive(Clone, Args)]
pub struct InitProject;

impl InitProject {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<InitProjectOutcomes>, ProjectCommandError> {
        let mut outcomes = Outcomes::new();

        let project_name = BrainName::new(
            context
                .project_name()
                .ok_or(ProjectCommandError::NoProject)?,
        );

        let client = context.client();

        let request = CreateBrainRequest {
            name: project_name.clone(),
        };

        match client.create_brain(request).await {
            Ok(info) => {
                context.store_ticket(project_name.as_str(), info.token.as_str())?;
                outcomes.emit(InitProjectOutcomes::BrainCreated(project_name.clone()));
            }
            Err(oneiros_client::Error::ServiceResponse(ref e)) if e.status == 409 => {
                outcomes.emit(InitProjectOutcomes::BrainAlreadyExists(project_name));
            }
            Err(error) => return Err(error.into()),
        }

        Ok(outcomes)
    }
}
