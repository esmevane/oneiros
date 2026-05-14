use std::io::BufRead;

use crate::*;

pub(crate) struct ProjectService;

impl ProjectService {
    pub(crate) async fn create(
        scope: &Scope<AtHost>,
        mailbox: &Mailbox,
        request: &CreateProject,
    ) -> Result<ProjectResponse, ProjectError> {
        let details = request.current()?;
        let project_name = details.name.clone().ok_or(ProjectError::MissingName)?;

        if let Ok(ProjectResponse::Found(_)) = Self::get(
            scope,
            &GetProject::builder_v1()
                .key(project_name.clone())
                .build()
                .into(),
        )
        .await
        {
            return Ok(ProjectResponse::ProjectAlreadyExists(
                ProjectAlreadyExistsResponse::builder_v1()
                    .project_name(project_name)
                    .build()
                    .into(),
            ));
        }

        let project = Project::builder().name(project_name.clone()).build();

        let new_event = NewEvent::builder()
            .data(Events::Project(ProjectEvents::ProjectCreated(
                ProjectCreated::builder_v1()
                    .project(project.clone())
                    .build()
                    .into(),
            )))
            .build();

        mailbox.tell(HostMessage::from(
            AppendHostLog::builder()
                .scope(scope.clone())
                .event(new_event)
                .build(),
        ));

        let stored = ProjectRepo::new(scope)
            .fetch(&project_name)
            .await?
            .ok_or_else(|| ProjectError::NotFound(project_name.clone()))?;

        let main_bookmark = Bookmark::builder()
            .project(project_name.clone())
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

        mailbox.tell(HostMessage::from(
            AppendHostLog::builder()
                .scope(scope.clone())
                .event(new_event)
                .build(),
        ));

        let platform = scope.config().platform();
        let project_dir = scope.config().data_dir.join(project_name.as_str());
        let bookmarks_dir = project_dir.join("bookmarks");
        platform.ensure_dir(&bookmarks_dir)?;

        let events_db = rusqlite::Connection::open(project_dir.join("events.db"))?;
        events_db.pragma_update(None, "journal_mode", "wal")?;
        EventLog::new(&events_db).init()?;
        drop(events_db);

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
                    .project_name(project_name.clone())
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
        platform.ensure_dir(&tickets_dir)?;

        let token_path = tickets_dir.join(format!("{project_name}.token"));
        platform.write(&token_path, format!("{token}"))?;

        Ok(ProjectResponse::Created(
            ProjectCreatedResponse::builder_v1()
                .project(stored)
                .token(token)
                .build()
                .into(),
        ))
    }

    pub(crate) async fn get(
        scope: &Scope<AtHost>,
        request: &GetProject,
    ) -> Result<ProjectResponse, ProjectError> {
        let GetProject::V1(lookup) = request;
        let repo = ProjectRepo::new(scope);
        let project = match &lookup.key {
            ResourceKey::Key(name) => repo
                .fetch(name)
                .await?
                .ok_or_else(|| ProjectError::NotFound(name.clone()))?,
            ResourceKey::Ref(token) => {
                let Ref::V0(resource) = token.inner().clone();
                match resource {
                    Resource::Project(id) => repo
                        .fetch_by_id(&id)
                        .await?
                        .ok_or(ProjectError::NotFoundById(id))?,
                    other => {
                        return Err(ProjectError::Resolve(ResolveError::WrongKind {
                            expected: "project",
                            got: other.label(),
                        }));
                    }
                }
            }
        };
        Ok(ProjectResponse::Found(
            ProjectFoundResponse::builder_v1()
                .project(project)
                .build()
                .into(),
        ))
    }

    pub(crate) async fn list(
        scope: &Scope<AtHost>,
        request: &ListProjects,
    ) -> Result<ProjectResponse, ProjectError> {
        let ListProjects::V1(listing) = request;
        let listed = ProjectRepo::new(scope).list(&listing.filters).await?;
        Ok(ProjectResponse::Listed(
            ProjectsResponse::builder_v1()
                .items(listed.items)
                .total(listed.total)
                .build()
                .into(),
        ))
    }

    /// Export all events to a JSONL file in the target directory.
    pub(crate) async fn export(
        scope: &Scope<AtBookmark>,
        request: &ExportProject,
    ) -> Result<ProjectResponse, ProjectError> {
        let details = request.current()?;
        let target_dir = &details.target;
        let project_name = &scope.config().project;
        let db = BookmarkDb::open(scope).await?;
        let events = EventLog::attached(&db).load_all()?;
        let storage = StorageStore::new(&db);

        let mut buffer = String::new();
        for event in &events {
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

        let platform = scope.config().platform();
        platform.ensure_dir(target_dir)?;

        let date = chrono::Utc::now().format("%Y-%m-%d");
        let file_name = format!("{project_name}-{date}-export.jsonl");
        let file_path = target_dir.join(file_name);

        platform.write(&file_path, buffer)?;

        Ok(ProjectResponse::WroteExport(
            WroteExportResponse::builder_v1()
                .path(file_path)
                .build()
                .into(),
        ))
    }

    /// Import events from a JSONL file and replay projections.
    pub(crate) async fn import(
        config: &Config,
        request: &ImportProject,
    ) -> Result<ProjectResponse, ProjectError> {
        let details = request.current()?;
        let platform = config.platform();
        let file = platform.open_file(&details.file)?;
        let reader = std::io::BufReader::new(file);
        let mut imported = 0usize;

        let db = BookmarkDb::open_with(
            &config.platform(),
            &config.project,
            &config.bookmark,
            config.database.limit_attached,
        )
        .await?;
        let log = EventLog::attached(&db);
        let projections = Projections::<ProjectCanon>::project();

        log.init()?;
        projections.migrate(&db)?;

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
        let replayed = projections.replay_project(&db, &log)?;

        Ok(ProjectResponse::Imported(
            ImportedResponse::builder_v1()
                .imported(imported as i64)
                .replayed(replayed as i64)
                .build()
                .into(),
        ))
    }

    /// Replay all events through projections, rebuilding read models.
    pub(crate) async fn replay(config: &Config) -> Result<ProjectResponse, ProjectError> {
        let platform = config.platform();
        let db_path = config.bookmark_db_path();
        if db_path.exists() {
            platform.remove_file(&db_path)?;
        }
        let _ = platform.remove_file(db_path.with_extension("db-wal"));
        let _ = platform.remove_file(db_path.with_extension("db-shm"));

        let db = BookmarkDb::open_with(
            &config.platform(),
            &config.project,
            &config.bookmark,
            config.database.limit_attached,
        )
        .await?;
        let projections = Projections::<ProjectCanon>::project();
        projections.migrate(&db)?;
        let log = EventLog::attached(&db);
        let replayed = projections.replay_project(&db, &log)?;

        Ok(ProjectResponse::Replayed(
            ReplayedResponse::builder_v1()
                .replayed(replayed as i64)
                .build()
                .into(),
        ))
    }
}
