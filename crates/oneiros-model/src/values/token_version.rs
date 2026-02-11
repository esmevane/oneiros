#[derive(serde::Serialize, serde::Deserialize)]
pub(super) enum TokenVersion {
    V0(super::claim::TokenClaims),
}
