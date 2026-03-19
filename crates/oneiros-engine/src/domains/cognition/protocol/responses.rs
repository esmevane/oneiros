use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitionAddedResult {
    pub id: CognitionId,
    pub ref_token: RefToken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum CognitionResponse {
    CognitionAdded(CognitionAddedResult),
    CognitionDetails(Cognition),
    Cognitions(Vec<Cognition>),
    NoCognitions,
}
