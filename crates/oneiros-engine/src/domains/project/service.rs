use std::io::BufRead;

use crate::*;

pub(crate) struct ProjectService;

impl ProjectService {
    pub(crate) async fn init(
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

        context
            .emit(BookmarkEvents::BookmarkCreated(BookmarkCreated {
                brain: brain_name.clone(),
                name: BookmarkName::main(),
            }))
            .await?;

        // Ensure brain directory and DB schema exist (mirrors legacy create_brain_db).
        let brain_dir = context.config.data_dir.join(brain_name.as_str());
        std::fs::create_dir_all(&brain_dir)?;
        let brain_db = rusqlite::Connection::open(brain_dir.join("brain.db"))?;
        EventLog::new(&brain_db).migrate()?;
        Projections::project().migrate(&brain_db)?;
        drop(brain_db);

        let all_filters = SearchFilters {
            limit: Limit(usize::MAX),
            offset: Offset(0),
        };
        let actors = ActorRepo::new(context).list(&all_filters).await?;

        let token = if let Some(actor) = actors.items.first() {
            match TicketService::create(
                context,
                &CreateTicket::builder()
                    .actor_id(actor.id)
                    .brain_name(brain_name.clone())
                    .build(),
            )
            .await?
            {
                TicketResponse::Created(ticket) => ticket.link.token,
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
    pub(crate) fn export(
        config: &Config,
        request: &ExportProject,
    ) -> Result<ProjectResponse, ProjectError> {
        let target_dir = &request.target;
        let project_name = &config.brain;
        let db = config.brain_db()?;
        let events = EventLog::new(&db).load_all()?;
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
    pub(crate) fn import(
        config: &Config,
        request: &ImportProject,
    ) -> Result<ProjectResponse, ProjectError> {
        let file = std::fs::File::open(&request.file)?;
        let reader = std::io::BufReader::new(file);
        let mut imported = 0usize;

        let db = config.brain_db()?;
        let log = EventLog::new(&db);

        // Batch all inserts in a single transaction — without this,
        // each INSERT is an implicit transaction with an fsync.
        db.execute_batch("BEGIN")?;

        let result = (|| -> Result<(), ProjectError> {
            for line in reader.lines() {
                let line = line?;

                if line.trim().is_empty() {
                    continue;
                }

                let event: StoredEvent = serde_json::from_str(&line)?;

                if let Events::Ephemeral(ephemeral) = &event.data {
                    match ephemeral {
                        EphemeralEvents::BlobStored(content) => {
                            StorageStore::new(&db).put_blob(content)?;
                        }
                    }
                } else {
                    log.import(&event)?;
                    imported += 1;
                }
            }
            Ok(())
        })();

        match result {
            Ok(()) => db.execute_batch("COMMIT")?,
            Err(e) => {
                let _ = db.execute_batch("ROLLBACK");
                return Err(e);
            }
        }

        let projections = Projections::<BrainCanon>::project();
        let replayed = projections.replay_brain(&db)?;

        Ok(ProjectResponse::Imported(ImportResult {
            imported: EventCount::new(imported as i64),
            replayed: EventCount::new(replayed as i64),
        }))
    }

    /// Replay all events through projections, rebuilding read models.
    pub(crate) fn replay(config: &Config) -> Result<ProjectResponse, ProjectError> {
        let projections = Projections::<BrainCanon>::project();
        let replayed = projections.replay_brain(&config.brain_db()?)?;

        Ok(ProjectResponse::Replayed(ReplayResult {
            replayed: EventCount::new(replayed as i64),
        }))
    }
}
