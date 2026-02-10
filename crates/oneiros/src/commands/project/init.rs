use clap::Args;
use oneiros_client::{CreateBrainRequest, HttpClient, ServiceClient};
use oneiros_outcomes::Outcomes;

use super::error::ProjectCommandError;
use super::init_outcomes::ProjectInitOutcomes;
use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ProjectInit;

impl ProjectInit {
    pub(crate) async fn run(
        &self,
        context: Context,
    ) -> Result<Outcomes<ProjectInitOutcomes>, ProjectCommandError> {
        let mut outcomes = Outcomes::new();

        let project_name = Label::new(
            context
                .project_name()
                .ok_or(ProjectCommandError::NoProject)?,
        );

        let client = HttpClient::new(context.socket_path());

        let request = CreateBrainRequest {
            name: project_name.clone(),
        };

        match client.create_brain(request).await {
            Ok(info) => {
                outcomes.emit(ProjectInitOutcomes::BrainCreated(info.name, info.path));
            }
            Err(oneiros_client::Error::ServiceResponse(ref e)) if e.status == 409 => {
                outcomes.emit(ProjectInitOutcomes::BrainAlreadyExists(project_name));
            }
            Err(error) => return Err(error.into()),
        }

        Ok(outcomes)
    }
}
