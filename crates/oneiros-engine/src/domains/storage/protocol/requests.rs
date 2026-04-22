use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetStorage {
    #[builder(into)]
    pub key: ResourceKey<StorageKey>,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct RemoveStorage {
    #[builder(into)]
    pub key: StorageKey,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct UploadStorage {
    #[builder(into)]
    pub key: StorageKey,
    #[arg(long, default_value = "")]
    #[builder(default, into)]
    pub description: Description,
    pub data: Vec<u8>,
}

#[derive(Builder, Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Args)]
pub struct ListStorage {
    #[command(flatten)]
    #[serde(flatten)]
    #[builder(default)]
    pub filters: SearchFilters,
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
