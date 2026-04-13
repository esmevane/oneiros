//! Response envelope — wraps domain responses with optional metadata.
//!
//! Handlers can attach pressure summaries or other cross-cutting concerns
//! to any response without changing domain types.

use serde::{Deserialize, Serialize};

use crate::{PressureSummary, RefToken};

/// A response envelope that wraps domain data with optional metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Response<T> {
    #[serde(flatten)]
    pub(crate) data: T,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) meta: Option<ResponseMeta>,
}

impl<T> Response<T> {
    pub(crate) fn new(data: T) -> Self {
        Self { data, meta: None }
    }

    pub(crate) fn with_meta(mut self, meta: ResponseMeta) -> Self {
        self.meta = Some(meta);
        self
    }

    pub(crate) fn with_ref_token(mut self, ref_token: RefToken) -> Self {
        self.meta
            .get_or_insert_with(ResponseMeta::default)
            .ref_token = Some(ref_token);
        self
    }

    pub(crate) fn meta(&self) -> ResponseMeta {
        self.meta.clone().unwrap_or_default()
    }
}

impl<T> From<T> for Response<T> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

/// Metadata attached to responses — pressure summaries, timing, etc.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct ResponseMeta {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) pressures: Vec<PressureSummary>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub(crate) ref_token: Option<RefToken>,
}

impl ResponseMeta {
    pub(crate) fn ref_token(&self) -> Option<RefToken> {
        self.ref_token.clone()
    }
}
