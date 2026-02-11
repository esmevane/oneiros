#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Id(pub uuid::Uuid);

impl Id {
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }

    #[deprecated]
    pub fn parse(s: &str) -> Option<Self> {
        uuid::Uuid::parse_str(s).ok().map(Self)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_nil()
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Unable to parse id, invalid contents: {0}")]
pub struct IdParseError(#[from] uuid::Error);

impl core::str::FromStr for Id {
    type Err = IdParseError;

    fn from_str(given_str: &str) -> Result<Self, Self::Err> {
        Ok(Self(uuid::Uuid::parse_str(given_str)?))
    }
}
