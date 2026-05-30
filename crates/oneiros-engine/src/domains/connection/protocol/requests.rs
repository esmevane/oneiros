use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum CreateConnection {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) nature: NatureName,
            pub(crate) from_ref: RefToken,
            pub(crate) to_ref: RefToken,
        }
    }
}

impl ClientRequest for CreateConnection {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/connections", self).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GetConnection {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<ConnectionId>,
        }
    }
}

impl ClientRequest for GetConnection {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let GetConnection::V1(lookup) = self;
        client.get(&format!("/connections/{}", lookup.key)).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListConnections {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long)]
            pub(crate) entity: Option<RefToken>,
            /// Lens expression — replaces ad-hoc filters with the unified
            /// query language. When set, entity is ignored and the lens
            /// drives selection end-to-end.
            #[arg(long)]
            #[builder(into)]
            pub(crate) lens: Option<String>,
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ClientRequest for ListConnections {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let ListConnections::V1(listing) = self;
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(entity) = &listing.entity {
            params.push(("entity", entity.to_string()));
        }

        if let Some(lens) = &listing.lens {
            params.push(("lens", lens.clone()));
        }

        params.push(("limit", listing.filters.limit.to_string()));
        params.push(("offset", listing.filters.offset.to_string()));

        let query = params
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join("&");

        client.get(&format!("/connections?{query}")).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum RemoveConnection {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) id: ConnectionId,
        }
    }
}

impl ClientRequest for RemoveConnection {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let RemoveConnection::V1(removal) = self;
        client.delete(&format!("/connections/{}", removal.id)).await
    }
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
