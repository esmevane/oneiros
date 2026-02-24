use clap::Args;
use oneiros_client::Client;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};
use oneiros_templates::IntrospectTemplate;

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum IntrospectError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum IntrospectOutcomes {
    #[outcome(message("Introspecting as '{}'...", .0.agent.name), prompt("{}", .0.prompt))]
    Introspecting(Introspection),
}

/// Summarize a session before context compaction.
#[derive(Clone, Args)]
pub struct IntrospectOp {
    /// The agent to introspect as.
    name: AgentName,
}

impl IntrospectOp {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<IntrospectOutcomes>, IntrospectError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let agent = client
            .introspect(&context.ticket_token()?, &self.name)
            .await?;
        let prompt = IntrospectTemplate::new(&agent).to_string();

        outcomes.emit(IntrospectOutcomes::Introspecting(Introspection {
            agent,
            prompt,
        }));

        Ok(outcomes)
    }
}
