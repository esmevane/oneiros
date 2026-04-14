//! Formal error response type — replaces ad-hoc `json!({ "error": ... })`
//! across all HTTP error boundaries.

use serde::{Deserialize, Serialize};

/// A structured error response for HTTP boundaries.
///
/// Every `IntoResponse` impl for domain errors produces this type,
/// giving clients a stable contract they can parse and extend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl ErrorResponse {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            code: None,
            detail: None,
        }
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_minimal() {
        let response = ErrorResponse::new("not found");
        let json = serde_json::to_value(&response).unwrap();

        assert_eq!(json, serde_json::json!({ "error": "not found" }));
    }

    #[test]
    fn serializes_with_code() {
        let response = ErrorResponse::new("not found").with_code("not_found");
        let json = serde_json::to_value(&response).unwrap();

        assert_eq!(
            json,
            serde_json::json!({ "error": "not found", "code": "not_found" })
        );
    }

    #[test]
    fn serializes_with_all_fields() {
        let response = ErrorResponse::new("not found")
            .with_code("not_found")
            .with_detail("Agent 'foo' does not exist");
        let json = serde_json::to_value(&response).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "error": "not found",
                "code": "not_found",
                "detail": "Agent 'foo' does not exist"
            })
        );
    }

    #[test]
    fn deserializes_minimal() {
        let json = r#"{"error":"bad request"}"#;
        let response: ErrorResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.error, "bad request");
        assert!(response.code.is_none());
        assert!(response.detail.is_none());
    }

    #[test]
    fn deserializes_with_all_fields() {
        let json = r#"{"error":"conflict","code":"conflict","detail":"already exists"}"#;
        let response: ErrorResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.error, "conflict");
        assert_eq!(response.code.as_deref(), Some("conflict"));
        assert_eq!(response.detail.as_deref(), Some("already exists"));
    }
}
