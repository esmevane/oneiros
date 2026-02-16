mod outcomes;

use clap::Args;
use oneiros_client::{AddExperienceRefRequest, Client};
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::{RefAddOutcomes, RefAddedResult};

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct RefAdd {
    /// The experience ID to add a reference to.
    experience_id: ExperienceId,

    /// The ID of the record to reference.
    record_id: Id,

    /// The kind of record being referenced.
    record_kind: RecordKind,

    /// Optional role label for this reference.
    #[arg(long)]
    role: Option<String>,
}

impl RefAdd {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<RefAddOutcomes>, ExperienceCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let token = context.ticket_token()?;

        let experience = client
            .add_experience_ref(
                &token,
                &self.experience_id,
                AddExperienceRefRequest {
                    record_id: self.record_id,
                    record_kind: self.record_kind.clone(),
                    role: self.role.as_ref().map(Label::new),
                },
            )
            .await?;

        let agents = client.list_agents(&token).await?;
        let gauge = agents
            .iter()
            .find(|a| a.id == experience.agent_id)
            .map(|agent| {
                // We have the agent name but need all experiences for that agent.
                // Since we just did the ref_add, we can't easily filter by agent name
                // without another call. Use the agent name with the experience we have.
                agent.name.clone()
            });

        let gauge_str = if let Some(agent_name) = gauge {
            let all = client
                .list_experiences(&token, Some(&agent_name), None)
                .await?;
            crate::gauge::experience_gauge(&agent_name, &all)
        } else {
            String::new()
        };

        outcomes.emit(RefAddOutcomes::RefAdded(RefAddedResult {
            id: experience.id,
            gauge: gauge_str,
        }));

        Ok(outcomes)
    }
}
