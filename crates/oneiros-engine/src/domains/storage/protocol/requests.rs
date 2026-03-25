use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetStorage {
    pub key: StorageKey,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct RemoveStorage {
    pub key: StorageKey,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct UploadStorage {
    pub key: StorageKey,
    #[arg(long, default_value = "")]
    pub description: Description,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum StorageRequest {
    Upload(UploadStorage),
    Get(GetStorage),
    List,
    Remove(RemoveStorage),
}
