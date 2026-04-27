use bon::Builder;
use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = ConnectionEventsType, display = "kebab-case")]
pub enum ConnectionEvents {
    ConnectionCreated(Connection),
    ConnectionRemoved(ConnectionRemoved),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_types_are_kebab_cased() {
        let cases = [
            (
                ConnectionEventsType::ConnectionCreated,
                "connection-created",
            ),
            (
                ConnectionEventsType::ConnectionRemoved,
                "connection-removed",
            ),
        ];
        for (event_type, expectation) in cases {
            assert_eq!(&event_type.to_string(), expectation);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConnectionRemoved {
    Current(ConnectionRemovedV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct ConnectionRemovedV1 {
    pub id: ConnectionId,
}

impl ConnectionRemoved {
    pub fn build_v1() -> ConnectionRemovedV1Builder {
        ConnectionRemovedV1::builder()
    }

    pub fn id(&self) -> ConnectionId {
        match self {
            Self::Current(v) => v.id,
        }
    }
}
