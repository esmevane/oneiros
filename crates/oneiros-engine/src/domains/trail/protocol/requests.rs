use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum TrailOf {
        #[derive(clap::Args)]
        V1 => {
            /// The entity ref to walk back from — events that touched this entity.
            #[builder(into)]
            #[serde(rename = "ref")]
            #[arg(name = "ref")]
            pub(crate) r#ref: RefToken,
        }
    }
}

impl ClientRequest for TrailOf {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let TrailOf::V1(of) = self;
        client.get(&format!("/trail/of/{}", of.r#ref)).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum TrailFrom {
        #[derive(clap::Args)]
        V1 => {
            /// The event id to walk forward from — entities this event emitted.
            #[builder(into)]
            pub(crate) event: EventId,
        }
    }
}

impl ClientRequest for TrailFrom {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let TrailFrom::V1(from) = self;
        client.get(&format!("/trail/from/{}", from.event)).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = TrailRequestType, display = "kebab-case")]
pub(crate) enum TrailRequest {
    TrailOf(TrailOf),
    TrailFrom(TrailFrom),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        assert_eq!(&TrailRequestType::TrailOf.to_string(), "trail-of");
        assert_eq!(&TrailRequestType::TrailFrom.to_string(), "trail-from");
    }
}
