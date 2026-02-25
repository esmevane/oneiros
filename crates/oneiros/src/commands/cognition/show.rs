use clap::Args;
use oneiros_client::Client;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowCognitionOutcomes {
    #[outcome(
        message("Cognition {}\n  Agent: {}\n  Texture: {}\n  Content: {}\n  Created: {}", .0.id, .0.agent_id, .0.texture, .0.content, .0.created_at),
        prompt("Does this connect to something? Trace it with `oneiros experience create <agent> <sensation> <description>`.")
    )]
    CognitionDetails(Cognition),
}

#[derive(Clone, Args)]
pub struct ShowCognition {
    /// The cognition ID (full UUID or 8+ character prefix).
    id: PrefixId,
}

impl ShowCognition {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowCognitionOutcomes>, CognitionCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let token = context.ticket_token()?;

        let id = match self.id.as_full_id() {
            Some(id) => CognitionId(id),
            None => {
                let all = client.list_cognitions(&token, None, None).await?;
                let ids: Vec<_> = all.iter().map(|c| c.id.0).collect();
                CognitionId(self.id.resolve(&ids)?)
            }
        };

        let cognition = client.get_cognition(&token, &id).await?;
        outcomes.emit(ShowCognitionOutcomes::CognitionDetails(cognition));

        Ok(outcomes)
    }
}
