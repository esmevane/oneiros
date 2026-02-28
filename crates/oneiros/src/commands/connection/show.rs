use clap::Args;
use oneiros_client::Client;
use oneiros_model::{Connection, ConnectionId};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize)]
#[serde(transparent)]
pub struct ConnectionDetail(pub Connection);

impl core::fmt::Display for ConnectionDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_detail())
    }
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowConnectionOutcomes {
    #[outcome(message("{0}"))]
    ConnectionDetails(ConnectionDetail),
}

#[derive(Clone, Args)]
pub struct ShowConnection {
    /// The connection ID (full UUID, 8+ character prefix, or ref:token).
    id: PrefixId,
}

impl ShowConnection {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowConnectionOutcomes>, ConnectionCommandError> {
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

        let connection = client.get_connection(&token, &id).await?;

        outcomes.emit(ShowConnectionOutcomes::ConnectionDetails(ConnectionDetail(
            connection,
        )));

        Ok(outcomes)
    }
}
