use clap::Args;
use oneiros_client::Client;
use oneiros_model::{ExperienceId, ExperienceRef, Label, RefToken};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct RefAddedResult {
    pub id: ExperienceId,
    #[serde(skip)]
    pub ref_token: RefToken,
    #[serde(skip)]
    pub gauge: String,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RefAddOutcomes {
    #[outcome(message("Reference added to experience: {}", .0.ref_token), prompt("{}", .0.gauge))]
    RefAdded(RefAddedResult),
}

#[derive(Clone, Args)]
pub struct RefAdd {
    /// The experience ID to add a reference to (full UUID, 8+ character prefix, or ref:token).
    experience_id: PrefixId,

    /// The entity reference (ref:base64url-encoded RefToken).
    entity: RefToken,

    /// Optional role label for this reference.
    #[arg(long)]
    role: Option<String>,
}

impl RefAdd {
    pub async fn run(
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

        let experience_ref = ExperienceRef::new(
            self.entity.clone().into_inner(),
            self.role.as_ref().map(Label::new),
        );

        let experience = client
            .add_experience_ref(&token, &experience_id, experience_ref)
            .await?;

        let agents = client.list_agents(&token).await?;
        let gauge = agents
            .iter()
            .find(|agent| agent.id == experience.agent_id)
            .map(|agent| agent.name.clone());

        let gauge_str = if let Some(agent_name) = gauge {
            let all = client
                .list_experiences(&token, Some(&agent_name), None)
                .await?;
            crate::gauge::experience_gauge(&agent_name, &all)
        } else {
            String::new()
        };

        let ref_token = experience.ref_token();

        outcomes.emit(RefAddOutcomes::RefAdded(RefAddedResult {
            id: experience.id,
            ref_token,
            gauge: gauge_str,
        }));

        Ok(outcomes)
    }
}
