use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct CreateConnection {
    #[builder(into)]
    pub(crate) nature: NatureName,
    pub(crate) from_ref: RefToken,
    pub(crate) to_ref: RefToken,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct GetConnection {
    #[builder(into)]
    pub(crate) id: ConnectionId,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct ListConnections {
    #[arg(long)]
    pub(crate) entity: Option<RefToken>,
    #[command(flatten)]
    #[serde(flatten)]
    #[builder(default)]
    pub(crate) filters: SearchFilters,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct RemoveConnection {
    #[builder(into)]
    pub(crate) id: ConnectionId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = ConnectionRequestType, display = "kebab-case")]
pub(crate) enum ConnectionRequest {
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
