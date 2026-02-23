use oneiros_outcomes::{PromptReporter, QuietReporter, Reportable, Reporter};
use serde::Serialize;

#[derive(Clone, Default, clap::ValueEnum)]
pub(crate) enum OutputFormat {
    /// Message with actionable prompt suggestions (default).
    #[default]
    Prompt,
    /// Message only, no prompts.
    Quiet,
    /// Structured JSON output.
    Json,
}

impl OutputFormat {
    pub(crate) fn structured_output<T: Reportable + Serialize>(&self, outcome: &T) {
        match self {
            Self::Prompt => PromptReporter.report(outcome),
            Self::Quiet => QuietReporter.report(outcome),
            Self::Json => {
                if let Some(mut value) = serde_json::to_value(outcome).ok()
                    && let Some(metadata) = serde_json::to_value(outcome.metadata()).ok()
                {
                    if let Some(value_as_object) = value.as_object_mut() {
                        value_as_object.insert("metadata".into(), metadata);

                        if let Ok(json) = serde_json::to_string(&value) {
                            println!("{json}");
                        }
                    } else if let Ok(json) = serde_json::to_string(&value) {
                        println!("{json}");
                    }
                }
            }
        }
    }
}
