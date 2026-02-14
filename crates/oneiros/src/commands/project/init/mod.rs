mod outcomes;

pub(crate) use outcomes::InitProjectOutcomes;

use clap::Args;
use oneiros_client::{Client, CreateBrainRequest};
use oneiros_outcomes::Outcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct InitProject;

impl InitProject {
    pub(crate) async fn run(
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
                context
                    .store_ticket(project_name.as_str(), info.token.as_str())
                    .map_err(ProjectCommandError::Io)?;
                outcomes.emit(InitProjectOutcomes::BrainCreated(
                    info.entity.name,
                    info.entity.path,
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
