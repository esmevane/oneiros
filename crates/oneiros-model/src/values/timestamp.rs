use chrono::{DateTime, ParseError, Utc};

#[derive(Debug, thiserror::Error)]
#[error("created_at invalid or malformed: {0}")]
pub struct TimestampParseError(#[from] ParseError);

#[derive(
    Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
#[serde(transparent)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    pub fn parse_str(created_at: impl AsRef<str>) -> Result<Self, TimestampParseError> {
        Ok(Self(created_at.as_ref().parse()?))
    }

    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn as_string(&self) -> String {
        self.0.to_rfc3339()
    }

    pub fn as_date_string(&self) -> String {
        self.0.format("%Y-%m-%d").to_string()
    }

    pub fn elapsed(&self) -> String {
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
