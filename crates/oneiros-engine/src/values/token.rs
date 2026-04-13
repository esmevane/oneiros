use super::claim::TokenClaims as Claim;
use super::token_version::TokenVersion;

#[derive(Debug, thiserror::Error)]
pub(crate) enum TokenError {
    #[error("Invalid token encoding")]
    Encoding,

    #[error("Invalid token format: {0}")]
    Format(#[from] postcard::Error),
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    lorosurgeon::Hydrate,
    lorosurgeon::Reconcile,
)]
#[serde(transparent)]
pub(crate) struct Token(pub(crate) String);

impl Token {
    /// Issue a new token encoding the given claims.
    pub(crate) fn issue(claims: Claim) -> Self {
        let versioned = TokenVersion::V0(claims);
        let bytes = postcard::to_allocvec(&versioned).expect("token serialization should not fail");
        Self(data_encoding::BASE32_NOPAD.encode(&bytes).to_lowercase())
    }

    /// Decode this token's claims.
    pub(crate) fn decode(&self) -> Result<Claim, TokenError> {
        let upper = self.0.to_uppercase();
        let bytes = data_encoding::BASE32_NOPAD
            .decode(upper.as_bytes())
            .map_err(|_| TokenError::Encoding)?;
        let versioned: TokenVersion = postcard::from_bytes(&bytes)?;
        let TokenVersion::V0(claims) = versioned;
        Ok(claims)
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl<T> From<T> for Token
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Self(value.as_ref().to_string())
    }
}

impl core::fmt::Display for Token {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}
