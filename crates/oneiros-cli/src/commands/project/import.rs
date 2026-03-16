use clap::Args;
use oneiros_outcomes::{Outcome, Outcomes};
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
    ) -> Result<(Outcomes<ImportProjectOutcomes>, Vec<PressureSummary>), ProjectCommandError> {
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

            let event: ImportEvent = serde_json::from_str(&line)?;
            events.push(event);
        }

        let response = context
            .client()
            .import_events(&context.ticket_token()?, events)
            .await?;
        let summaries = response.pressure_summaries();
        let result: ImportResponse = response.data()?;

        outcomes.emit(ImportProjectOutcomes::Imported(result.imported));
        outcomes.emit(ImportProjectOutcomes::Replayed(result.replayed));

        Ok((outcomes, summaries))
    }
}
