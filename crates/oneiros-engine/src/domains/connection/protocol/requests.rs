use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum CreateConnection {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub nature: NatureName,
            pub from_ref: RefToken,
            pub to_ref: RefToken,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum GetConnection {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub key: ResourceKey<ConnectionId>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ListConnections {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long)]
            pub entity: Option<RefToken>,
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub filters: SearchFilters,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum RemoveConnection {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub id: ConnectionId,
        }
    }
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
