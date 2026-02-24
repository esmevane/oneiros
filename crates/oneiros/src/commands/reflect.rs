use clap::Args;
use oneiros_client::Client;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};
use oneiros_templates::ReflectTemplate;

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum ReflectError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ReflectOutcomes {
    #[outcome(message("Reflecting as '{}'...", .0.agent.name), prompt("{}", .0.prompt))]
    Reflecting(Reflection),
}

/// Reflect on a significant event during a session.
#[derive(Clone, Args)]
pub struct ReflectOp {
    /// The agent to reflect as.
    name: AgentName,
}

impl ReflectOp {
    pub async fn run(&self, context: &Context) -> Result<Outcomes<ReflectOutcomes>, ReflectError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let agent = client.reflect(&context.ticket_token()?, &self.name).await?;
        let prompt = ReflectTemplate::new(&agent).to_string();

        outcomes.emit(ReflectOutcomes::Reflecting(Reflection { agent, prompt }));

        Ok(outcomes)
    }
}
