mod error;
mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;
use oneiros_templates::ReflectTemplate;

pub(crate) use error::ReflectError;
pub(crate) use outcomes::ReflectOutcomes;

use crate::*;

/// Reflect on a significant event during a session.
#[derive(Clone, Args)]
pub(crate) struct ReflectOp {
    /// The agent to reflect as.
    name: AgentName,
}

impl ReflectOp {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ReflectOutcomes>, ReflectError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let agent = client.reflect(&context.ticket_token()?, &self.name).await?;
        let prompt = ReflectTemplate::new(&agent).to_string();

        outcomes.emit(ReflectOutcomes::Reflecting(Reflection { agent, prompt }));

        Ok(outcomes)
    }
}
