use oneiros_model::{Connection, ConnectionId, Identity};
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize)]
#[serde(transparent)]
pub struct ConnectionList(pub Vec<Identity<ConnectionId, Connection>>);

impl core::fmt::Display for ConnectionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = self
            .0
            .iter()
            .map(|connection| format!("{connection}"))
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{display}")
    }
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListConnectionsOutcomes {
    #[outcome(message("No connections found."))]
    NoConnections,

    #[outcome(message("{0}"))]
    Connections(ConnectionList),
}
