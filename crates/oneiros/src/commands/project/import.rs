use clap::Args;
use oneiros_client::ImportEvent;
use oneiros_outcomes::{Outcome, Outcomes};
use serde_json::Value;
use std::io::BufRead;
use std::path::PathBuf;

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ImportProjectOutcomes {
    #[outcome(message("Importing from '{}'.", .0.to_string_lossy()))]
    ReadingFile(PathBuf),
    #[outcome(message("Imported {0} events."))]
    Imported(usize),
    #[outcome(message("Replayed {0} events."))]
    Replayed(usize),
}

#[derive(Clone, Args)]
pub struct ImportProject {
    /// Path to the jsonl export file to import.
    pub file: PathBuf,
}

impl ImportProject {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ImportProjectOutcomes>, ProjectCommandError> {
        let mut outcomes = Outcomes::new();

        let path = if self.file.is_relative() {
            std::env::current_dir()?.join(&self.file)
        } else {
            self.file.clone()
        };

        outcomes.emit(ImportProjectOutcomes::ReadingFile(path.clone()));

        let file = std::fs::File::open(&path)?;
        let reader = std::io::BufReader::new(file);
        let mut events = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            let envelope: Value = serde_json::from_str(&line)?;

            let timestamp = envelope
                .get("timestamp")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string();

            let data = envelope.get("data").cloned().unwrap_or(Value::Null);

            events.push(ImportEvent { timestamp, data });
        }

        let response = context
            .client()
            .import_events(&context.ticket_token()?, events)
            .await?;

        outcomes.emit(ImportProjectOutcomes::Imported(response.imported));
        outcomes.emit(ImportProjectOutcomes::Replayed(response.replayed));

        Ok(outcomes)
    }
}
