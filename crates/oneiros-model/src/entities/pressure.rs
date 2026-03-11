use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum PressureConstructionError {
    #[error("invalid pressure id: {0}")]
    InvalidId(IdParseError),
    #[error("invalid agent id: {0}")]
    InvalidAgentId(IdParseError),
    #[error("invalid gauge data: {0}")]
    InvalidGauge(#[from] serde_json::Error),
    #[error("invalid updated_at timestamp: {0}")]
    InvalidUpdatedAt(#[from] TimestampParseError),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct Pressure {
    pub id: PressureId,
    pub agent_id: AgentId,
    pub urge: UrgeName,
    pub data: Gauge,
    pub updated_at: Timestamp,
}

impl Pressure {
    pub fn urgency(&self) -> f64 {
        self.data.urgency()
    }

    pub fn construct_from_db(
        (id, agent_id, urge, data, updated_at): (
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
        ),
    ) -> Result<Self, PressureConstructionError> {
        Ok(Pressure {
            id: id
                .as_ref()
                .parse()
                .map_err(PressureConstructionError::InvalidId)?,
            agent_id: agent_id
                .as_ref()
                .parse()
                .map_err(PressureConstructionError::InvalidAgentId)?,
            urge: UrgeName::new(urge),
            data: serde_json::from_str(data.as_ref())?,
            updated_at: Timestamp::parse_str(updated_at)?,
        })
    }
}

impl core::fmt::Display for Pressure {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} {:>5.1}% {}",
            self.urge,
            self.urgency() * 100.0,
            self.updated_at
        )
    }
}

domain_id!(PressureId);
