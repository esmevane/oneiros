use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = StorageResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum StorageResponse {
    StorageSet(Response<StorageEntry>),
    StorageDetails(Response<StorageEntry>),
    Entries(Listed<Response<StorageEntry>>),
    NoEntries,
    StorageRemoved(StorageKey),
}
