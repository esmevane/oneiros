mod error;
mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;
use oneiros_templates::IntrospectTemplate;

pub(crate) use error::IntrospectError;
pub(crate) use outcomes::IntrospectOutcomes;

use crate::*;

/// Summarize a session before context compaction.
#[derive(Clone, Args)]
pub(crate) struct IntrospectOp {
    /// The agent to introspect as.
    name: AgentName,
}

impl IntrospectOp {
    pub(crate) async fn run(
        &self,
        context: Context,
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
