use std::io::BufRead;

use crate::*;

pub struct ProjectService;

impl ProjectService {
    pub async fn init(
        context: &SystemContext,
        request: &InitProject,
    ) -> Result<ProjectResponse, ProjectError> {
        let brain_name = request
            .name
            .clone()
            .unwrap_or_else(|| context.config.brain.clone());

        if let Ok(BrainResponse::Found(_)) = BrainService::get(
            context,
            &GetBrain::builder().name(brain_name.clone()).build(),
        )
        .await
        {
            return Ok(ProjectResponse::BrainAlreadyExists(brain_name));
        }

        BrainService::create(
            context,
            &CreateBrain::builder().name(brain_name.clone()).build(),
        )
        .await?;

        let actors = ActorRepo::new(context).list().await?;

        let token = if let Some(actor) = actors.first() {
            match TicketService::create(
                context,
                &CreateTicket::builder()
                    .actor_id(actor.id)
                    .brain_name(brain_name.clone())
                    .build(),
            )
            .await?
            {
                TicketResponse::Created(ticket) => ticket.token,
                _ => return Err(ProjectError::Missing),
            }
        } else {
            return Err(ProjectError::Missing);
        };

        let tickets_dir = context.config.data_dir.join("tickets");

        std::fs::create_dir_all(&tickets_dir)?;

        let token_path = tickets_dir.join(format!("{brain_name}.token"));

        std::fs::write(&token_path, format!("{token}"))?;

        Ok(ProjectResponse::Initialized(InitResult {
            brain_name,
            token,
        }))
    }

    /// Export all events to a JSONL file in the target directory.
    ///
    /// When a `StorageSet` event is encountered, an ephemeral `BlobStored`
    /// event is prepended carrying the binary content. This makes the export
    /// portable — the receiving brain materializes the blob at import time
    /// without persisting the ephemeral event to the log.
    pub fn export(
        context: &ProjectContext,
        request: &ExportProject,
    ) -> Result<ProjectResponse, ProjectError> {
        let target_dir = &request.target;
        let project_name = context.brain_name();
        let events = EventLog::new(&context.db()?).load_all()?;
        let db = context.db()?;
        let storage = StorageStore::new(&db);

        let mut buffer = String::new();
        for event in &events {
            // Synthesize ephemeral BlobStored events for storage portability.
            if let Events::Storage(StorageEvents::StorageSet(entry)) = &event.data
                && let Ok(Some(blob)) = storage.get_blob(&entry.hash)
            {
                let synthetic = StoredEvent::builder()
                    .id(EventId::new())
                    .sequence(0)
                    .created_at(event.created_at)
                    .source(event.source)
                    .data(Events::Ephemeral(EphemeralEvents::BlobStored(blob)))
                    .build();

                buffer.push_str(&serde_json::to_string(&synthetic)?);
                buffer.push('\n');
            }

            buffer.push_str(&serde_json::to_string(event)?);
            buffer.push('\n');
        }

        std::fs::create_dir_all(target_dir)?;

        let date = chrono::Utc::now().format("%Y-%m-%d");
        let file_name = format!("{project_name}-{date}-export.jsonl");
        let file_path = target_dir.join(file_name);

        std::fs::write(&file_path, buffer)?;

        Ok(ProjectResponse::WroteExport(ExportPath::new(file_path)))
    }

    /// Import events from a JSONL file and replay projections.
    ///
    /// Ephemeral events (like BlobStored) are materialized directly
    /// at the import boundary — they never enter the event log.
    /// Domain events are persisted normally, then all projections
    /// are replayed to rebuild the read models.
    pub fn import(
        context: &ProjectContext,
        request: &ImportProject,
    ) -> Result<ProjectResponse, ProjectError> {
        let file = std::fs::File::open(&request.file)?;
        let reader = std::io::BufReader::new(file);
        let mut imported = vec![];

        let db = context.db()?;
        let log = EventLog::new(&db);

        for line in reader.lines() {
            let line = line?;

            if line.trim().is_empty() {
                continue;
            }

            let event: StoredEvent = serde_json::from_str(&line)?;

            if let Events::Ephemeral(ephemeral) = &event.data {
                match ephemeral {
                    EphemeralEvents::BlobStored(content) => {
                        StorageStore::new(&context.db()?).put_blob(content)?;
                    }
                }
            } else {
                log.import(&event)?;
                imported.push(event);
            }
        }

        let replayed = context.projections.replay(&db)?;

        Ok(ProjectResponse::Imported(ImportResult {
            imported: EventCount::new(imported.len() as i64),
            replayed: EventCount::new(replayed as i64),
        }))
    }

    /// Replay all events through projections, rebuilding read models.
    pub fn replay(context: &ProjectContext) -> Result<ProjectResponse, ProjectError> {
        let replayed = context.projections.replay(&context.db()?)?;

        Ok(ProjectResponse::Replayed(ReplayResult {
            replayed: EventCount::new(replayed as i64),
        }))
    }
}
