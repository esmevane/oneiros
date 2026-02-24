use clap::Args;
use oneiros_client::{Client, CreateBrainRequest};
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};
use std::path::PathBuf;

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum InitProjectOutcomes {
    #[outcome(message("Brain '{0}' created at {}.", .1.display()))]
    BrainCreated(BrainName, PathBuf),
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

        let client = Client::new(context.socket_path());

        let request = CreateBrainRequest {
            name: project_name.clone(),
        };

        match client.create_brain(request).await {
            Ok(info) => {
                context.store_ticket(project_name.as_str(), info.token.as_str())?;
                outcomes.emit(InitProjectOutcomes::BrainCreated(
                    info.entity.name.clone(),
                    info.entity.path.clone(),
                ));
            }
            Err(oneiros_client::Error::ServiceResponse(ref e)) if e.status == 409 => {
                outcomes.emit(InitProjectOutcomes::BrainAlreadyExists(project_name));
            }
            Err(error) => return Err(error.into()),
        }

        Ok(outcomes)
    }
}
