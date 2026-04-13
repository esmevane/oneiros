/// How the caller wants output formatted.
#[derive(
    Debug, Clone, Default, PartialEq, clap::ValueEnum, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum OutputMode {
    /// Structured data — machines, acceptance tests, --output json.
    Json,
    /// Human-readable summary — terminal users.
    Text,
    /// Agent-facing rendered template — AI consumers.
    #[default]
    Prompt,
}

impl std::fmt::Display for OutputMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Text => write!(f, "text"),
            Self::Prompt => write!(f, "prompt"),
        }
    }
}
