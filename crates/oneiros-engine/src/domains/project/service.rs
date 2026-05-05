use std::io::BufRead;

use crate::*;

pub struct ProjectService;

impl ProjectService {
    pub async fn init(
        scope: &Scope<AtHost>,
        mailbox: &Mailbox,
        request: &InitProject,
    ) -> Result<ProjectResponse, ProjectError> {
        let details = request.current()?;
        let brain_name = details
            .name
            .clone()
            .unwrap_or_else(|| scope.config().brain.clone());

        if let Ok(BrainResponse::Found(_)) = BrainService::get(
            scope,
            &GetBrain::builder_v1()
                .key(brain_name.clone())
                .build()
                .into(),
        )
        .await
        {
            return Ok(ProjectResponse::BrainAlreadyExists(
                BrainAlreadyExistsResponse::builder_v1()
                    .brain_name(brain_name)
                    .build()
                    .into(),
            ));
        }

        BrainService::create(
            scope,
            mailbox,
            &CreateBrain::builder_v1()
                .name(brain_name.clone())
                .build()
                .into(),
        )
        .await?;

        let main_bookmark = Bookmark::builder()
            .brain(brain_name.clone())
            .name(BookmarkName::main())
            .build();

        let new_event = NewEvent::builder()
            .data(Events::Bookmark(BookmarkEvents::BookmarkCreated(
                BookmarkCreated::builder_v1()
                    .bookmark(main_bookmark)
                    .build()
                    .into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        // Create the brain's database layout:
        //   {brain_dir}/events.db         — event log (append-only)
        //   {brain_dir}/bookmarks/main.db — projection tables for the default bookmark
        let brain_dir = scope.config().data_dir.join(brain_name.as_str());
        let bookmarks_dir = brain_dir.join("bookmarks");
        std::fs::create_dir_all(&bookmarks_dir)?;

        // Event log — standalone, no ATTACH needed during init.
        let events_db = rusqlite::Connection::open(brain_dir.join("events.db"))?;
        events_db.pragma_update(None, "journal_mode", "wal")?;
        EventLog::new(&events_db).init()?;
        drop(events_db);

        // Default bookmark projections — standalone during init.
        let bookmark_db = rusqlite::Connection::open(bookmarks_dir.join("main.db"))?;
        bookmark_db.pragma_update(None, "journal_mode", "wal")?;
        Projections::project().migrate(&bookmark_db)?;
        drop(bookmark_db);

        let all_filters = SearchFilters {
            limit: Limit(usize::MAX),
            offset: Offset(0),
        };
        let actors = ActorRepo::new(scope).list(&all_filters).await?;

        let token = if let Some(actor) = actors.items.first() {
            match TicketService::create(
                scope,
                mailbox,
                &CreateTicket::builder_v1()
                    .actor_id(actor.id)
                    .brain_name(brain_name.clone())
                    .build()
                    .into(),
            )
            .await?
            {
                TicketResponse::Created(TicketCreatedResponse::V1(created)) => {
                    created.ticket.link.token
                }
                _ => return Err(ProjectError::Missing),
            }
        } else {
            return Err(ProjectError::Missing);
        };

        let tickets_dir = scope.config().data_dir.join("tickets");

        std::fs::create_dir_all(&tickets_dir)?;

        let token_path = tickets_dir.join(format!("{brain_name}.token"));

        std::fs::write(&token_path, format!("{token}"))?;

        Ok(ProjectResponse::Initialized(
            InitializedResponse::builder_v1()
                .brain_name(brain_name)
                .token(token)
                .build()
                .into(),
        ))
    }

    /// Export all events to a JSONL file in the target directory.
    ///
    /// When a `StorageSet` event is encountered, an ephemeral `BlobStored`
    /// event is prepended carrying the binary content. This makes the export
    /// portable — the receiving brain materializes the blob at import time
    /// without persisting the ephemeral event to the log.
    pub async fn export(
        scope: &Scope<AtBookmark>,
        request: &ExportProject,
    ) -> Result<ProjectResponse, ProjectError> {
        let details = request.current()?;
        let target_dir = &details.target;
        let project_name = &scope.config().brain;
        let db = BookmarkDb::open(scope).await?;
        let events = EventLog::attached(&db).load_all()?;
        let storage = StorageStore::new(&db);

        let mut buffer = String::new();
        for event in &events {
            // Synthesize ephemeral BlobStored events for storage portability.
            if let Event::Known(Events::Storage(StorageEvents::StorageSet(set))) = &event.data
                && let Ok(current) = set.current()
                && let Ok(Some(blob)) = storage.get_blob(&current.entry.hash)
            {
                let synthetic = StoredEvent::builder()
                    .id(EventId::new())
                    .sequence(0)
                    .created_at(event.created_at)
                    .source(event.source)
                    .data(Event::Ephemeral(EphemeralEvents::BlobStored(blob)))
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

        Ok(ProjectResponse::WroteExport(
            WroteExportResponse::builder_v1()
                .path(file_path)
                .build()
                .into(),
        ))
    }

    /// Import events from a JSONL file and replay projections.
    ///
    /// Ephemeral events (like BlobStored) are materialized directly
    /// at the import boundary — they never enter the event log.
    /// Domain events are persisted normally, then all projections
    /// are replayed to rebuild the read models.
    /// Import is self-bootstrapping — the destination brain may not yet
    /// be in the projection (system init without project init), so we
    /// can't compose a `Scope<AtBookmark>` here. Take the platform +
    /// brain + bookmark directly and open the bookmark DB via the
    /// underlying primitive.
    pub async fn import(
        config: &Config,
        request: &ImportProject,
    ) -> Result<ProjectResponse, ProjectError> {
        let details = request.current()?;
        let file = std::fs::File::open(&details.file)?;
        let reader = std::io::BufReader::new(file);
        let mut imported = 0usize;

        let db = BookmarkDb::open_with(&config.platform(), &config.brain, &config.bookmark).await?;
        let log = EventLog::attached(&db);
        let projections = Projections::<BrainCanon>::project();

        // Import is self-bootstrapping: a destination brain may have seen
        // `system init` without `project init`, leaving the events DB and
        // bookmark DB unmigrated. Running the migrations here is idempotent
        // (CREATE TABLE IF NOT EXISTS) and makes import the correctness
        // gate the versioning story relies on.
        log.init()?;
        projections.migrate(&db)?;

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

                if let Event::Ephemeral(ephemeral) = &event.data {
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

        let log = EventLog::attached(&db);
        let replayed = projections.replay_brain(&db, &log)?;

        Ok(ProjectResponse::Imported(
            ImportedResponse::builder_v1()
                .imported(imported as i64)
                .replayed(replayed as i64)
                .build()
                .into(),
        ))
    }

    /// Replay all events through projections, rebuilding read models.
    ///
    /// Deletes the bookmark DB file first, then recreates it with the
    /// current schema via `migrate()` before replaying all events.
    /// This ensures schema changes are picked up — `CREATE TABLE IF NOT EXISTS`
    /// alone cannot add columns to existing tables. The event log
    /// (`events.db`) is untouched; projection data is always derivable
    /// from events.
    ///
    /// Takes `&Config` rather than `&Scope<AtBookmark>` because the
    /// bookmark DB file may be deleted mid-call — composing a scope
    /// would fail the existence check.
    pub async fn replay(config: &Config) -> Result<ProjectResponse, ProjectError> {
        // Ensure a clean schema by deleting the old bookmark DB.
        // WAL sidecar files are silently cleaned up if present.
        let db_path = config.bookmark_db_path();
        if db_path.exists() {
            std::fs::remove_file(&db_path)?;
        }
        let _ = std::fs::remove_file(db_path.with_extension("db-wal"));
        let _ = std::fs::remove_file(db_path.with_extension("db-shm"));

        // Open a fresh DB, create tables with current schema, then replay.
        let db = BookmarkDb::open_with(&config.platform(), &config.brain, &config.bookmark).await?;
        let projections = Projections::<BrainCanon>::project();
        projections.migrate(&db)?;
        let log = EventLog::attached(&db);
        let replayed = projections.replay_brain(&db, &log)?;

        Ok(ProjectResponse::Replayed(
            ReplayedResponse::builder_v1()
                .replayed(replayed as i64)
                .build()
                .into(),
        ))
    }
}
