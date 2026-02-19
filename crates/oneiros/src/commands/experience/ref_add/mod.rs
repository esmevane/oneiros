mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::{RefAddOutcomes, RefAddedResult};

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct RefAdd {
    /// The experience ID to add a reference to (full UUID or 8+ character prefix).
    experience_id: PrefixId,

    /// The ID of the record to reference (full UUID or 8+ character prefix).
    record_id: PrefixId,

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

        let experience_id = match self.experience_id.as_full_id() {
            Some(id) => ExperienceId(id),
            None => {
                let all = client.list_experiences(&token, None, None).await?;
                let ids: Vec<_> = all.iter().map(|e| e.id.0).collect();
                ExperienceId(self.experience_id.resolve(&ids)?)
            }
        };

        let record_id = match self.record_id.as_full_id() {
            Some(id) => id,
            None => {
                let ids = super::list_ids_for_kind(&client, &token, &self.record_kind).await?;
                self.record_id.resolve(&ids)?
            }
        };

        let experience = client
            .add_experience_ref(
                &token,
                &experience_id,
                RecordRef::identified(
                    record_id,
                    self.record_kind.clone(),
                    self.role.as_ref().map(Label::new),
                ),
            )
            .await?;

        let agents = client.list_agents(&token).await?;
        let gauge = agents
            .iter()
            .find(|a| a.id == experience.agent_id)
            .map(|agent| agent.name.clone());

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
