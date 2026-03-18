//! Response envelope — wraps domain responses with optional metadata.
//!
//! Handlers can attach pressure summaries or other cross-cutting concerns
//! to any response without changing domain types.

use serde::{Deserialize, Serialize};

use crate::PressureSummary;

/// A response envelope that wraps domain data with optional metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    #[serde(flatten)]
    pub data: T,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ResponseMeta>,
}

impl<T> Response<T> {
    pub fn new(data: T) -> Self {
        Self { data, meta: None }
    }

    pub fn with_meta(mut self, meta: ResponseMeta) -> Self {
        self.meta = Some(meta);
        self
    }
}

impl<T> From<T> for Response<T> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

/// Metadata attached to responses — pressure summaries, timing, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMeta {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub pressures: Vec<PressureSummary>,
}
