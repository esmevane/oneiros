use crate::Reportable;

/// Serializable metadata extracted from a [`Reportable`].
#[derive(serde::Serialize)]
pub struct ReportableMetadata {
    pub level: &'static str,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}

impl ReportableMetadata {
    pub fn from_reportable<T: Reportable>(reportable: &T) -> Self {
        Self {
            level: match reportable.level() {
                tracing::Level::TRACE => "trace",
                tracing::Level::DEBUG => "debug",
                tracing::Level::INFO => "info",
                tracing::Level::WARN => "warn",
                tracing::Level::ERROR => "error",
            },
            message: reportable.message(),
            prompt: reportable.prompt(),
        }
    }
}
