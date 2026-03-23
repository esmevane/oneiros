/// How the caller wants output formatted.
#[derive(Debug, Clone, Default)]
pub enum OutputMode {
    /// Structured data — machines, acceptance tests, --output json.
    #[default]
    Json,
    /// Human-readable summary — terminal users.
    Text,
    /// Agent-facing rendered template — AI consumers.
    Prompt,
}
