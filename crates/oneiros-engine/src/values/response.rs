//! Response envelope — wraps domain responses with optional metadata.
//!
//! Handlers can attach pressure summaries, reference tokens, or
//! navigational hints to any response without changing domain types.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// A response envelope that wraps domain data with optional metadata.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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

    pub(crate) fn with_hints(mut self, hints: Vec<Hint>) -> Self {
        if !hints.is_empty() {
            self.meta.get_or_insert_with(ResponseMeta::default).hints = hints;
        }
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

/// Metadata attached to responses — pressure summaries, hints, etc.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub(crate) struct ResponseMeta {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) pressures: Vec<PressureSummary>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub(crate) ref_token: Option<RefToken>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) hints: Vec<Hint>,
}

impl ResponseMeta {
    pub(crate) fn ref_token(&self) -> Option<RefToken> {
        self.ref_token.clone()
    }

    pub(crate) fn hints(&self) -> &[Hint] {
        &self.hints
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn response_meta_skips_empty_hints() {
        let meta = ResponseMeta::default();
        let json = serde_json::to_string(&meta).unwrap();
        assert!(!json.contains("hints"));
    }

    #[test]
    fn response_meta_includes_hints_when_present() {
        let meta = ResponseMeta {
            hints: vec![Hint::suggest("search", "Find related entities")],
            ..Default::default()
        };
        let json = serde_json::to_string(&meta).unwrap();
        assert!(json.contains("\"hints\""));
        assert!(json.contains("\"suggest\""));
    }

    #[test]
    fn with_hints_skips_empty_vec() {
        let response = Response::new("data").with_hints(vec![]);
        assert!(response.meta.is_none());
    }

    #[test]
    fn with_hints_attaches_to_meta() {
        let hints = vec![Hint::suggest("search", "Find things")];
        let response = Response::new("data").with_hints(hints.clone());
        assert_eq!(response.meta().hints(), &hints[..]);
    }
}
