use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum GetStorage {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub key: ResourceKey<StorageKey>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum RemoveStorage {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub key: StorageKey,
        }
    }
}

// UploadStorage carries a `Vec<u8>` payload, which does not satisfy
// clap's parsing requirements — the CLI uses an `--file` path instead
// and reads the bytes before constructing the request.
versioned! {
    #[derive(JsonSchema)]
    pub enum UploadStorage {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub key: StorageKey,
            #[builder(default, into)] pub description: Description,
            pub data: Vec<u8>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ListStorage {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub filters: SearchFilters,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = StorageRequestType, display = "kebab-case")]
pub enum StorageRequest {
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
