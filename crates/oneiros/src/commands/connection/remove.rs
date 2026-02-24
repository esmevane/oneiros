use clap::Args;
use oneiros_client::Client;
use oneiros_model::ConnectionId;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RemoveConnectionOutcomes {
    #[outcome(message("Connection {0} removed."))]
    ConnectionRemoved(ConnectionId),
}

#[derive(Clone, Args)]
pub struct RemoveConnection {
    /// The connection ID (full UUID or 8+ character prefix).
    id: PrefixId,
}

impl RemoveConnection {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<RemoveConnectionOutcomes>, ConnectionCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let token = context.ticket_token()?;

        let id = match self.id.as_full_id() {
            Some(id) => ConnectionId(id),
            None => {
                let all = client.list_connections(&token, None, None).await?;
                let ids: Vec<_> = all.iter().map(|c| c.id.0).collect();
                ConnectionId(self.id.resolve(&ids)?)
            }
        };

        client.remove_connection(&token, &id).await?;
        outcomes.emit(RemoveConnectionOutcomes::ConnectionRemoved(id));

        Ok(outcomes)
    }
}
