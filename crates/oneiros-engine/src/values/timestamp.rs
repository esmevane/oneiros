use chrono::{DateTime, ParseError, Utc};

#[derive(Debug, thiserror::Error)]
#[error("created_at invalid or malformed: {0}")]
pub struct TimestampParseError(#[from] ParseError);

#[derive(
    Clone,
    Copy,
    Debug,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(transparent)]
pub(crate) struct Timestamp(DateTime<Utc>);

impl Timestamp {
    pub(crate) fn parse_str(created_at: impl AsRef<str>) -> Result<Self, TimestampParseError> {
        Ok(Self(created_at.as_ref().parse()?))
    }

    pub(crate) fn now() -> Self {
        Self(Utc::now())
    }

    pub(crate) fn as_string(&self) -> String {
        self.0.to_rfc3339()
    }

    pub(crate) fn as_date_string(&self) -> String {
        self.0.format("%Y-%m-%d").to_string()
    }

    pub(crate) fn elapsed(&self) -> String {
        let elapsed = Utc::now().signed_duration_since(self.0);
        let secs = elapsed.num_seconds();

        if secs < 0 {
            "just now".to_string()
        } else if secs < 60 {
            format!("{secs}s ago")
        } else if secs < 3600 {
            format!("{}m ago", secs / 60)
        } else if secs < 86400 {
            format!("{}h ago", secs / 3600)
        } else {
            format!("{}d ago", secs / 86400)
        }
    }
}

impl core::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl lorosurgeon::Hydrate for Timestamp {
    fn hydrate_string(given_string: &str) -> Result<Self, lorosurgeon::HydrateError> {
        Self::parse_str(given_string).map_err(|_| {
            lorosurgeon::HydrateError::unexpected("timestamp string", "unexpected string")
        })
    }
}

impl lorosurgeon::Reconcile for Timestamp {
    type Key = lorosurgeon::NoKey;
    fn reconcile<R: lorosurgeon::Reconciler>(
        &self,
        r: R,
    ) -> Result<(), lorosurgeon::ReconcileError> {
        r.str(&self.to_string())
    }
}
