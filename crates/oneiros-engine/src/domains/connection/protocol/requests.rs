use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct CreateConnection {
    #[builder(into)]
    pub nature: NatureName,
    pub from_ref: RefToken,
    pub to_ref: RefToken,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetConnection {
    #[builder(into)]
    pub id: ConnectionId,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct ListConnections {
    #[arg(long)]
    pub entity: Option<RefToken>,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct RemoveConnection {
    #[builder(into)]
    pub id: ConnectionId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = ConnectionRequestType, display = "kebab-case")]
pub enum ConnectionRequest {
    CreateConnection(CreateConnection),
    GetConnection(GetConnection),
    ListConnections(ListConnections),
    RemoveConnection(RemoveConnection),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (ConnectionRequestType::CreateConnection, "create-connection"),
            (ConnectionRequestType::GetConnection, "get-connection"),
            (ConnectionRequestType::ListConnections, "list-connections"),
            (ConnectionRequestType::RemoveConnection, "remove-connection"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
