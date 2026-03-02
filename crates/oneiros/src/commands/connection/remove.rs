use clap::Args;
use oneiros_model::{ConnectionId, RefToken};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct ConnectionRemovedResult {
    pub id: ConnectionId,
    #[serde(skip)]
    pub ref_token: RefToken,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RemoveConnectionOutcomes {
    #[outcome(message("Connection {} removed.", .0.ref_token))]
    ConnectionRemoved(ConnectionRemovedResult),
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
    ) -> Result<Outcomes<RemoveConnectionOutcomes>, ConnectionCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();
        let token = context.ticket_token()?;

        let id = match self.id.as_full_id() {
            Some(id) => ConnectionId(id),
            None => {
                let all = client.list_connections(&token, None, None).await?;
                let ids: Vec<_> = all.iter().map(|c| c.id.0).collect();
                ConnectionId(self.id.resolve(&ids)?)
            }
        };

        let ref_token = RefToken::new(oneiros_model::Ref::connection(id));

        client.remove_connection(&token, &id).await?;
        outcomes.emit(RemoveConnectionOutcomes::ConnectionRemoved(
            ConnectionRemovedResult { id, ref_token },
        ));

        Ok(outcomes)
    }
}
