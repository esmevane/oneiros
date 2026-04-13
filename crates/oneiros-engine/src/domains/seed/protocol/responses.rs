use kinded::Kinded;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = SeedResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum SeedResponse {
    SeedComplete,
    AgentsSeedComplete,
}
