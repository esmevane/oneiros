mod outcomes;

use clap::Args;
use oneiros_client::{Client, CreateExperienceRequest};
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::CreateExperienceOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct CreateExperience {
    /// The agent who is creating this experience.
    agent: AgentName,

    /// The sensation of experience being created.
    sensation: SensationName,

    /// A description of the experience.
    description: Content,

    /// References to cognitive records in the format: id:kind or id:kind:role
    #[arg(long = "ref")]
    refs: Vec<String>,
}

impl CreateExperience {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<CreateExperienceOutcomes>, ExperienceCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        // Parse refs from the command line format
        let mut refs = Vec::new();
        for ref_str in &self.refs {
            let parts: Vec<&str> = ref_str.split(':').collect();
            if parts.len() < 2 || parts.len() > 3 {
                return Err(ExperienceCommandError::InvalidRefFormat(format!(
                    "Expected format 'id:kind' or 'id:kind:role', got: {}",
                    ref_str
                )));
            }

            let id = parts[0].parse::<Id>().map_err(|e| {
                ExperienceCommandError::InvalidRefFormat(format!("Invalid id: {}", e))
            })?;
            let kind = parts[1].parse::<RecordKind>().map_err(|e| {
                ExperienceCommandError::InvalidRefFormat(format!("Invalid kind: {}", e))
            })?;
            let role = if parts.len() == 3 {
                Some(Label::new(parts[2]))
            } else {
                None
            };

            refs.push(RecordRef { id, kind, role });
        }

        let experience = client
            .create_experience(
                &context.ticket_token()?,
                CreateExperienceRequest {
                    agent: self.agent.clone(),
                    sensation: self.sensation.clone(),
                    description: self.description.clone(),
                    refs,
                },
            )
            .await?;
        outcomes.emit(CreateExperienceOutcomes::ExperienceCreated(experience.id));

        Ok(outcomes)
    }
}
