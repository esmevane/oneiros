use clap::Args;
use oneiros_client::Client;
use oneiros_model::ExperienceId;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct ExperienceCreatedResult {
    pub id: ExperienceId,
    #[serde(skip)]
    pub gauge: String,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum CreateExperienceOutcomes {
    #[outcome(message("Experience created: {}", .0.id), prompt("{}", .0.gauge))]
    ExperienceCreated(ExperienceCreatedResult),
}

#[derive(Clone, Args)]
pub struct CreateExperience {
    /// The agent who is creating this experience.
    agent: AgentName,

    /// The sensation of experience being created.
    sensation: SensationName,

    /// A description of the experience.
    description: Description,

    /// References to cognitive records in the format: id:kind or id:kind:role
    #[arg(long = "ref")]
    refs: Vec<String>,
}

impl CreateExperience {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<CreateExperienceOutcomes>, ExperienceCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let token = context.ticket_token()?;

        let refs = resolve_refs(&self.refs, &client, &token).await?;

        let experience = client
            .create_experience(
                &token,
                CreateExperienceRequest {
                    agent: self.agent.clone(),
                    sensation: self.sensation.clone(),
                    description: self.description.clone(),
                    refs,
                },
            )
            .await?;

        let all = client
            .list_experiences(&token, Some(&self.agent), None)
            .await?;
        let gauge = crate::gauge::experience_gauge(&self.agent, &all);

        outcomes.emit(CreateExperienceOutcomes::ExperienceCreated(
            ExperienceCreatedResult {
                id: experience.id,
                gauge,
            },
        ));

        Ok(outcomes)
    }
}

async fn resolve_refs(
    ref_strings: &[String],
    client: &Client,
    token: &oneiros_model::Token,
) -> Result<Vec<RecordRef>, ExperienceCommandError> {
    let mut refs = Vec::new();

    for ref_str in ref_strings {
        let parts: Vec<&str> = ref_str.split(':').collect();
        if parts.len() < 2 || parts.len() > 3 {
            return Err(ExperienceCommandError::InvalidRefFormat(format!(
                "Expected format 'id:kind' or 'id:kind:role', got: {}",
                ref_str
            )));
        }

        let prefix = parts[0]
            .parse::<PrefixId>()
            .map_err(|e| ExperienceCommandError::InvalidRefFormat(format!("Invalid id: {}", e)))?;
        let kind = parts[1].parse::<RecordKind>().map_err(|e| {
            ExperienceCommandError::InvalidRefFormat(format!("Invalid kind: {}", e))
        })?;
        let role = if parts.len() == 3 {
            Some(Label::new(parts[2]))
        } else {
            None
        };

        let id = match prefix.as_full_id() {
            Some(id) => id,
            None => {
                let ids = super::ops::list_ids_for_kind(client, token, &kind).await?;
                prefix.resolve(&ids)?
            }
        };

        refs.push(RecordRef::identified(id, kind, role));
    }

    Ok(refs)
}
