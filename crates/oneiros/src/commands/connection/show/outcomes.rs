use oneiros_model::{Connection, ConnectionId, Identity};
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize)]
#[serde(transparent)]
pub struct ConnectionDetail(pub Identity<ConnectionId, Connection>);

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
