use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = ConnectionEventsType, display = "kebab-case")]
pub enum ConnectionEvents {
    ConnectionCreated(ConnectionCreated),
    ConnectionRemoved(ConnectionRemoved),
}

versioned! {
    pub enum ConnectionCreated {
        V1 => {
            #[serde(flatten)] pub connection: Connection,
        }
    }
}

versioned! {
    pub enum ConnectionRemoved {
        V1 => {
            pub id: ConnectionId,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_connection() -> Connection {
        Connection::builder()
            .from_ref(Ref::cognition(CognitionId::new()))
            .to_ref(Ref::memory(MemoryId::new()))
            .nature("references")
            .build()
    }

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

    #[test]
    fn connection_created_wire_format_is_flat() {
        let event =
            ConnectionEvents::ConnectionCreated(ConnectionCreated::V1(ConnectionCreatedV1 {
                connection: sample_connection(),
            }));

        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["type"], "connection-created");
        assert!(
            json["data"].get("connection").is_none(),
            "flatten must elide the connection envelope on the wire"
        );
        assert_eq!(json["data"]["nature"], "references");
        assert!(json["data"].get("from_ref").is_some());
        assert!(json["data"].get("to_ref").is_some());
        assert!(
            json["data"].get("V1").is_none(),
            "V1 layer must not appear on the wire"
        );
    }
}
