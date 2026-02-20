use oneiros_outcomes::Outcome;
use std::path::PathBuf;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ReplayBrainOutcomes {
    #[outcome(message("Read {0} events from brain database."))]
    EventsRead(usize),
    #[outcome(message("Rewrote {0} events with content-addressed IDs."))]
    EventsRewritten(usize),
    #[outcome(message("Backed up original database to {}.", .0.display()))]
    BackupCreated(PathBuf),
    #[outcome(message("Created fresh brain database at {}.", .0.display()))]
    FreshDbCreated(PathBuf),
    #[outcome(message("Replayed {0} events through projections."))]
    EventsReplayed(usize),
    #[outcome(message("Replay complete."))]
    Complete,
}
