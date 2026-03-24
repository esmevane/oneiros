use clap::Args;
use oneiros_model::{Connection, ConnectionId};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RemoveConnectionOutcomes {
    #[outcome(message("Connection {0} removed."), prompt("Connection {0} removed."))]
    ConnectionRemoved(ConnectionId),
}

#[derive(Clone, Args)]
pub struct RemoveConnection {
    /// The connection ID (full UUID, 8+ character prefix, or ref:token).
    id: PrefixId,
}

impl RemoveConnection {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<(Outcomes<RemoveConnectionOutcomes>, Vec<PressureSummary>), ConnectionCommandError>
    {
        let mut outcomes = Outcomes::new();

        let client = context.client();
        let token = context.ticket_token()?;

        let id = match self.id.as_full_id() {
            Some(id) => ConnectionId(id),
            None => {
                let all: Vec<Connection> =
                    client.list_connections(&token, None, None).await?.data()?;
                let ids: Vec<_> = all.iter().map(|c| c.id.0).collect();
                ConnectionId(self.id.resolve(&ids)?)
            }
        };

        client.remove_connection(&token, &id).await?;
        outcomes.emit(RemoveConnectionOutcomes::ConnectionRemoved(id));

        Ok((outcomes, vec![]))
    }
}
