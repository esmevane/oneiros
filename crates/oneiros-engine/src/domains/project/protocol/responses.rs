use serde::Serialize;

use crate::*;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ProjectResponse {
    BrainCreated(BrainName),
    BrainAlreadyExists(BrainName),
}
