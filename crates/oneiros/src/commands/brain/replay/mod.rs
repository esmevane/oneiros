pub(crate) mod outcomes;

pub(crate) use outcomes::ReplayBrainOutcomes;

use clap::Args;
use oneiros_db::Database;
use oneiros_outcomes::Outcomes;
use oneiros_service::BRAIN_PROJECTIONS;
use oneiros_service::replay::rewrite_events;

use super::BrainCommandError;

#[derive(Clone, Args)]
pub(crate) struct ReplayBrain;

impl ReplayBrain {
    pub(crate) async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<Outcomes<ReplayBrainOutcomes>, BrainCommandError> {
        let mut outcomes = Outcomes::new();

        // Resolve the brain database path from the current project.
        let project_name = context.project_name().ok_or(BrainCommandError::NoProject)?;
        let brain_path = context
            .data_dir
            .join("brains")
            .join(format!("{project_name}.db"));

        if !brain_path.exists() {
            return Err(BrainCommandError::NoBrainDb(brain_path));
        }

        // Step 1: Read all events from the brain database.
        let source_db = Database::open_brain(&brain_path)?;
        let events = source_db.read_events()?;
        let event_count = events.len();
        outcomes.emit(ReplayBrainOutcomes::EventsRead(event_count));

        // Step 2: Rewrite events with content-addressed IDs.
        let rewritten = rewrite_events(events)?;
        outcomes.emit(ReplayBrainOutcomes::EventsRewritten(rewritten.len()));

        // Drop the source DB connection before renaming.
        drop(source_db);

        // Step 3: Backup the original database.
        let backup_path = brain_path.with_extension("db.backup");
        std::fs::rename(&brain_path, &backup_path)?;
        outcomes.emit(ReplayBrainOutcomes::BackupCreated(backup_path));

        // Step 4: Create a fresh brain database at the original path.
        // This gets the current schema from brain.sql.
        let fresh_db = Database::create_brain_db(&brain_path)?;
        outcomes.emit(ReplayBrainOutcomes::FreshDbCreated(brain_path));

        // Step 5: Replay rewritten events through projections.
        let mut replayed = 0;
        for event in &rewritten {
            fresh_db.log_event(event, BRAIN_PROJECTIONS)?;
            replayed += 1;
        }
        outcomes.emit(ReplayBrainOutcomes::EventsReplayed(replayed));
        outcomes.emit(ReplayBrainOutcomes::Complete);

        Ok(outcomes)
    }
}
