use kinded::Kinded;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = SeedResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SeedResponse {
    SeedComplete,
    AgentsSeedComplete,
}
