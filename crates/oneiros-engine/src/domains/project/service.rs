use std::io::BufRead;
use std::path::Path;

use crate::*;

pub struct ProjectService;

impl ProjectService {
    pub async fn init(
        ctx: &SystemContext,
        brain_name: BrainName,
    ) -> Result<ProjectResponse, ProjectError> {
        if let Ok(BrainResponse::Found(_)) = BrainService::get(ctx, &brain_name) {
            return Ok(ProjectResponse::BrainAlreadyExists(brain_name));
        }

        BrainService::create(ctx, brain_name.clone()).await?;

        let actors = ctx.with_db(|conn| ActorRepo::new(conn).list())?;

        if let Some(actor) = actors.first() {
            TicketService::create(ctx, actor.id.clone(), brain_name.clone()).await?;
        }

        Ok(ProjectResponse::BrainCreated(brain_name))
    }

    /// Export all events to a JSONL file in the target directory.
    ///
    /// When a `StorageSet` event is encountered, a synthetic `BlobStored` event
    /// is prepended carrying the binary content. This makes the export portable —
    /// the receiving brain can materialize the blob from the event stream.
    /// The `BlobStored` event is transient: the import projection materializes
    /// the blob into the `blob` table, then deletes the event.
    pub fn export(
        ctx: &ProjectContext,
        target_dir: &Path,
        project_name: &BrainName,
    ) -> Result<ProjectResponse, ProjectError> {
        let events = ctx.bus().load_events()?;

        let mut buffer = String::new();
        for event in &events {
            // Synthesize BlobStored events for storage portability.
            if let Events::Storage(StorageEvents::StorageSet(entry)) = &event.data {
                if let Ok(Some(blob)) =
                    ctx.with_db(|conn| StorageRepo::new(conn).get_blob(&entry.hash))
                {
                    let synthetic = ExportEvent {
                        id: Id::new().to_string(),
                        sequence: 0,
                        timestamp: event.created_at.to_rfc3339(),
                        source: event.source,
                        data: Events::Storage(StorageEvents::BlobStored(blob)),
                    };
                    buffer.push_str(&serde_json::to_string(&synthetic)?);
                    buffer.push('\n');
                }
            }

            let export: ExportEvent = event.clone().into();
            buffer.push_str(&serde_json::to_string(&export)?);
            buffer.push('\n');
        }

        std::fs::create_dir_all(target_dir)?;

        let date = chrono::Utc::now().format("%Y-%m-%d");
        let file_name = format!("{project_name}-{date}-export.jsonl");
        let file_path = target_dir.join(file_name);

        std::fs::write(&file_path, buffer)?;

        Ok(ProjectResponse::WroteExport(file_path))
    }

    /// Import events from a JSONL file and replay projections.
    pub fn import(ctx: &ProjectContext, file_path: &Path) -> Result<ProjectResponse, ProjectError> {
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        let mut imported = 0;

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            let event: ImportEvent = serde_json::from_str(&line)?;
            let event = event.with_source(Source::default());
            ctx.bus().import(&event)?;
            imported += 1;
        }

        let replayed = ctx.replay()?;

        Ok(ProjectResponse::Imported(ImportResult {
            imported,
            replayed,
        }))
    }

    /// Replay all events through projections, rebuilding read models.
    pub fn replay(ctx: &ProjectContext) -> Result<ProjectResponse, ProjectError> {
        let replayed = ctx.replay()?;
        Ok(ProjectResponse::Replayed(ReplayResult { replayed }))
    }
}
