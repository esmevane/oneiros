use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response as AxumResponse},
};
use oneiros_model::*;
use oneiros_service::OneirosService;

#[derive(Debug, thiserror::Error)]
pub enum OneirosContextError {
    #[error(transparent)]
    Service(#[from] oneiros_service::Error),
    #[error("Missing authorization header")]
    NoAuthHeader,
    #[error("Invalid auth header")]
    InvalidAuthHeader,
}

impl IntoResponse for OneirosContextError {
    fn into_response(self) -> AxumResponse {
        let body = serde_json::json!({ "error": self.to_string() });
        (StatusCode::UNAUTHORIZED, axum::Json(body)).into_response()
    }
}

pub struct OneirosContext {
    service: OneirosService,
}

impl OneirosContext {
    /// Dispatch a protocol request and return a Response with pressure meta.
    pub(crate) fn dispatch(&self, request: impl Into<Requests>) -> Result<Response, crate::Error> {
        Ok(self.service.dispatch(request)?)
    }

    /// Set storage content — bypasses the dispatch enum for binary data.
    pub(crate) fn set_storage(&self, request: SetStorageRequest) -> Result<Response, crate::Error> {
        Ok(self.service.set_storage(request)?)
    }

    /// Get raw storage content bytes.
    pub(crate) fn get_storage_content(&self, key: &StorageKey) -> Result<Vec<u8>, crate::Error> {
        Ok(self.service.get_storage_content(key)?)
    }
}

impl FromRequestParts<OneirosService> for OneirosContext {
    type Rejection = crate::Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &OneirosService,
    ) -> Result<Self, Self::Rejection> {
        let token_string = parts
            .headers
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or(OneirosContextError::NoAuthHeader)?
            .strip_prefix("Bearer ")
            .ok_or(OneirosContextError::InvalidAuthHeader)?;

        Ok(Self {
            service: state.upgrade(token_string)?,
        })
    }
}
