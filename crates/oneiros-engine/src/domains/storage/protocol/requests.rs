use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GetStorage {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<StorageKey>,
        }
    }
}

impl ClientRequest for GetStorage {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let GetStorage::V1(lookup) = self;
        let path = match &lookup.key {
            ResourceKey::Key(key) => StorageRef::encode(key).to_string(),
            ResourceKey::Ref(token) => token.to_string(),
        };
        client.get(&format!("/storage/{path}")).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum RemoveStorage {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: StorageKey,
        }
    }
}

impl ClientRequest for RemoveStorage {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let RemoveStorage::V1(removal) = self;
        let ref_key = StorageRef::encode(&removal.key);
        client.delete(&format!("/storage/{ref_key}")).await
    }
}

// UploadStorage carries a `Vec<u8>` payload, which does not satisfy
// clap's parsing requirements — the CLI uses an `--file` path instead
// and reads the bytes before constructing the request.
versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum UploadStorage {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: StorageKey,
            #[builder(default, into)] pub(crate) description: Description,
            pub(crate) data: Vec<u8>,
        }
    }
}

impl ClientRequest for UploadStorage {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/storage", self).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListStorage {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ClientRequest for ListStorage {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let ListStorage::V1(listing) = self;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset
        );
        client.get(&format!("/storage?{query}")).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(
    kind = StorageRequestType,
    display = "kebab-case",
    attrs(
        expect(
            clippy::enum_variant_names,
            reason = "We use these for `type` notation in serde"
        )
    )
)]
#[expect(
    clippy::enum_variant_names,
    reason = "We use these for `type` notation in serde"
)]
pub(crate) enum StorageRequest {
    UploadStorage(UploadStorage),
    GetStorage(GetStorage),
    ListStorage(ListStorage),
    RemoveStorage(RemoveStorage),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (StorageRequestType::UploadStorage, "upload-storage"),
            (StorageRequestType::GetStorage, "get-storage"),
            (StorageRequestType::ListStorage, "list-storage"),
            (StorageRequestType::RemoveStorage, "remove-storage"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
