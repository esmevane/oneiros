use crate::*;

pub(crate) struct BookmarkService;

impl BookmarkService {
    pub(crate) async fn create(
        state: &ServerState,
        project: &ProjectName,
        request: &CreateBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let CreateBookmark::V1(creation) = request;
        let name = &creation.name;
        let from = state.canons().active_bookmark(project)?;

        let mut event_ids = creation.event_ids.clone();

        if let Some(slice_name) = &creation.from_slice {
            let host_scope = ComposeScope::new(state.config().clone()).host()?;
            let host_db = HostDb::open(&host_scope).await?;
            let lens_expr: String = host_db.query_row(
                "SELECT lens_expr FROM slices WHERE name = ?1",
                rusqlite::params![slice_name.to_string()],
                |row| row.get(0),
            )?;
            let bookmark_scope = ComposeScope::new(state.config().clone())
                .bookmark(project.clone(), from.clone())?;
            let selection = LensService::select(
                &bookmark_scope,
                state.canons(),
                &format!("events_for({lens_expr})"),
            )
            .await
            .map_err(|e| BookmarkError::InvalidUri(e.to_string()))?;
            event_ids = selection.event_ids();
        }

        state.canons().fork_project(project, name)?;

        // Create the new bookmark's DB and replay the source events into it.
        // If event_ids is non-empty, only those events are replayed (scoped fork).
        Self::create_bookmark_db(state.config(), project, name, &event_ids)?;

        let bookmark = Bookmark::builder()
            .project(project.clone())
            .name(name.clone())
            .build();

        let scope = ComposeScope::new(state.config().clone()).host()?;
        let new_event = NewEvent::builder()
            .data(Events::Bookmark(BookmarkEvents::BookmarkForked(
                BookmarkForked::builder_v1()
                    .bookmark(bookmark.clone())
                    .from(from.clone())
                    .build()
                    .into(),
            )))
            .build();
        state.mailbox().tell(HostMessage::from(
            AppendHostLog::builder()
                .scope(scope)
                .event(new_event)
                .build(),
        ));

        Ok(BookmarkResponse::Forked(
            BookmarkForkedResponse::builder_v1()
                .bookmark(bookmark)
                .from(from)
                .build()
                .into(),
        ))
    }

    pub(crate) async fn switch(
        state: &ServerState,
        project: &ProjectName,
        request: &SwitchBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let SwitchBookmark::V1(switching) = request;
        let name = &switching.name;
        state.canons().switch_project(project, name)?;

        let event = BookmarkSwitched::builder_v1()
            .project(project.clone())
            .name(name.clone())
            .build();

        let scope = ComposeScope::new(state.config().clone()).host()?;
        let new_event = NewEvent::builder()
            .data(Events::Bookmark(BookmarkEvents::BookmarkSwitched(
                event.into(),
            )))
            .build();
        state.mailbox().tell(HostMessage::from(
            AppendHostLog::builder()
                .scope(scope)
                .event(new_event)
                .build(),
        ));

        Ok(BookmarkResponse::Switched(
            BookmarkSwitchedResponse::builder_v1()
                .project(project.clone())
                .name(name.clone())
                .build()
                .into(),
        ))
    }

    pub(crate) async fn merge(
        state: &ServerState,
        project: &ProjectName,
        request: &MergeBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let MergeBookmark::V1(merging) = request;
        let source = &merging.source;
        let target = state.canons().active_bookmark(project)?;
        state.canons().merge_project(project, source, &target)?;

        Self::replay_bookmark(state.config(), project, &target)?;

        let event = BookmarkMerged::builder_v1()
            .project(project.clone())
            .source(source.clone())
            .target(target.clone())
            .build();

        let scope = ComposeScope::new(state.config().clone()).host()?;
        let new_event = NewEvent::builder()
            .data(Events::Bookmark(BookmarkEvents::BookmarkMerged(
                event.into(),
            )))
            .build();
        state.mailbox().tell(HostMessage::from(
            AppendHostLog::builder()
                .scope(scope)
                .event(new_event)
                .build(),
        ));

        Ok(BookmarkResponse::Merged(
            BookmarkMergedResponse::builder_v1()
                .project(project.clone())
                .source(source.clone())
                .target(target)
                .build()
                .into(),
        ))
    }

    pub(crate) async fn list(
        state: &ServerState,
        project: &ProjectName,
        request: &ListBookmarks,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let ListBookmarks::V1(listing) = request;
        let scope = ComposeScope::new(state.config().clone()).host()?;
        let listed = BookmarkRepo::new(&scope)
            .list(project, &listing.filters)
            .await?;
        Ok(BookmarkResponse::Bookmarks(listed))
    }

    /// Share a bookmark by minting a distribution ticket scoped to it and
    /// composing an `oneiros://` URI that carries the host's address plus
    /// the ticket's link. Delegates to `TicketService::issue` for the
    /// actual ticket minting.
    pub(crate) async fn share(
        state: &ServerState,
        project: &ProjectName,
        request: &ShareBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let ShareBookmark::V1(sharing) = request;
        let name = &sharing.name;
        let actor_id = &sharing.actor_id;
        let scope = ComposeScope::new(state.config().clone()).host()?;
        let mailbox = state.mailbox();
        let all = SearchFilters {
            limit: Limit(usize::MAX),
            offset: Offset(0),
        };

        let bookmarks = BookmarkRepo::new(&scope).list(project, &all).await?;
        let bookmark = bookmarks
            .items
            .iter()
            .find(|b| b.name == *name)
            .ok_or_else(|| BookmarkError::NotFound(name.clone()))?
            .clone();

        let project_record = ProjectRepo::new(&scope)
            .get(project)
            .await?
            .ok_or_else(|| BookmarkError::ProjectNotFound(project.clone()))?;

        let resolved_actor_id = match actor_id {
            Some(id) => *id,
            None => {
                let actors = ActorRepo::new(&scope).list(&all).await?;
                actors
                    .items
                    .first()
                    .map(|a| a.id)
                    .ok_or(BookmarkError::NoActor)?
            }
        };

        let target = Ref::bookmark(bookmark.id);
        let ticket = TicketService::issue(
            &scope,
            mailbox,
            project,
            &project_record,
            resolved_actor_id,
            target,
        )
        .await?;

        let identity = state.host_identity();
        let peer_link = PeerLink::new(identity.address, ticket.link.clone());
        let uri = OneirosUri::Peer(peer_link).to_string();

        let new_event = NewEvent::builder()
            .data(Events::Bookmark(BookmarkEvents::BookmarkShared(
                BookmarkShared::builder_v1()
                    .project(project.clone())
                    .bookmark(name.clone())
                    .ticket_id(ticket.id)
                    .shared_by(resolved_actor_id)
                    .build()
                    .into(),
            )))
            .build();
        mailbox.tell(HostMessage::from(
            AppendHostLog::builder()
                .scope(scope)
                .event(new_event)
                .build(),
        ));

        Ok(BookmarkResponse::Shared(BookmarkShareResult {
            ticket,
            uri,
        }))
    }

    /// Follow a bookmark via an `oneiros://` or `ref:` URI.
    pub(crate) async fn follow(
        state: &ServerState,
        project: &ProjectName,
        request: &FollowBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let FollowBookmark::V1(following) = request;
        let uri = &following.uri;
        let name = &following.name;
        let scope = ComposeScope::new(state.config().clone()).host()?;
        let mailbox = state.mailbox();
        let parsed: OneirosUri = uri
            .parse()
            .map_err(|e: OneirosUriError| BookmarkError::InvalidUri(e.to_string()))?;

        let source = match parsed {
            OneirosUri::Ref(r) => FollowSource::Local(r),
            OneirosUri::Link(link) => FollowSource::Local(link.target),
            OneirosUri::Peer(peer_link) => {
                let key = PeerKey::from_bytes(*peer_link.host.inner().id.as_bytes());
                PeerService::ensure(&scope, mailbox, key, peer_link.host.clone()).await?;
                FollowSource::Peer(peer_link)
            }
        };

        let bookmark = Bookmark::builder()
            .project(project.clone())
            .name(name.clone())
            .build();

        let new_event = NewEvent::builder()
            .data(Events::Bookmark(BookmarkEvents::BookmarkCreated(
                BookmarkCreated::builder_v1()
                    .bookmark(bookmark)
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

        state.canons().fork_project(project, name)?;

        // Create the new bookmark's DB (empty — collect will populate it).
        Self::create_bookmark_db(state.config(), project, name, &[])?;

        let follow =
            FollowService::create(&scope, mailbox, project.clone(), name.clone(), source).await?;
        Ok(BookmarkResponse::Followed(follow))
    }

    /// Collect from a follow's source.
    pub(crate) async fn collect(
        state: &ServerState,
        project: &ProjectName,
        request: &CollectBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let CollectBookmark::V1(collection) = request;
        let name = &collection.name;
        let scope = ComposeScope::new(state.config().clone()).host()?;
        let mailbox = state.mailbox();
        let follow = FollowService::for_bookmark(&scope, project, name)
            .await?
            .ok_or_else(|| BookmarkError::FollowNotFound(name.clone()))?;

        match follow.source.clone() {
            FollowSource::Local(_) => {
                let checkpoint = Checkpoint::empty();
                FollowService::advance(&scope, mailbox, follow.id, checkpoint.clone(), 0).await?;
                Ok(BookmarkResponse::Collected(BookmarkCollectResult {
                    follow_id: follow.id,
                    events_received: 0,
                    checkpoint,
                }))
            }
            FollowSource::Peer(peer_link) => {
                Self::collect_from_peer(state, project, &follow, peer_link).await
            }
        }
    }

    /// Collect from a peer via Merkle diff on the chronicle HAMT.
    ///
    /// The local-side ingestion flows through the bus as `Import`
    /// messages. The inbound actor handles the events.db insert
    /// (insert-or-ignore by id), then forwards `Stored` to the
    /// bookmark + chronicle children — which project and record
    /// without distinction from locally-emitted events.
    async fn collect_from_peer(
        state: &ServerState,
        project: &ProjectName,
        follow: &Follow,
        peer_link: PeerLink,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        let mailbox = state.mailbox();
        let bridge = state.bridge();

        // Get the bookmark's chronicle — read-only here, used for the
        // Merkle diff. The chronicle actor updates it on each Stored
        // notification from the inbound actor.
        let chronicle = state
            .canons()
            .bookmark_chronicle(project, &follow.bookmark)?;
        let local_root = chronicle.root()?;

        // Build a local resolver from the host DB's ChronicleStore.
        // Opens its own connection per resolve call — Send-safe across
        // the async diff, and fast with WAL mode (~20 resolves per tree walk).
        {
            let db = state.config().host_db()?;
            ChronicleStore::new(&db).migrate()?;
        }
        let local_resolve = {
            let config = state.config().clone();
            move |hash: &ContentHash| -> Option<LedgerNode> {
                let db = config.host_db().ok()?;
                ChronicleStore::new(&db).get(hash)
            }
        };

        // Phase 1: Merkle diff — walk the peer's chronicle tree,
        // comparing against our local chronicle to find missing events.
        let diff_result = bridge
            .diff(
                &peer_link.host,
                &peer_link.link,
                local_root.as_ref(),
                &local_resolve,
            )
            .await
            .map_err(|e: BridgeError| BookmarkError::InvalidUri(e.to_string()))?;

        let events_received = diff_result.missing.len() as u64;

        // Phase 2: Fetch missing events and dispatch each as an
        // `Import` through the bus. The inbound actor handles
        // events.db insert and notifies the project actor; bookmark
        // + chronicle children project / record naturally.
        if !diff_result.missing.is_empty() {
            let event_ids: Vec<String> = diff_result
                .missing
                .iter()
                .map(|id| id.to_string())
                .collect();

            let fetch_request = BridgeRequest::BridgeFetchEvents(BridgeFetchEvents {
                link: peer_link.link.clone(),
                event_ids,
            });

            let events = bridge
                .fetch_events(&peer_link.host, &fetch_request)
                .await
                .map_err(|error: BridgeError| BookmarkError::InvalidUri(error.to_string()))?;

            let bookmark_scope = ComposeScope::new(state.config().clone())
                .bookmark(project.clone(), follow.bookmark.clone())?;

            // Capture the imported event ids — we'll wait until the
            // chronicle (and therefore the bookmark actor processing
            // the same FIFO) has seen them all.
            let expected_ids: std::collections::HashSet<String> =
                events.iter().map(|event| event.id.to_string()).collect();

            for event in events {
                mailbox.tell(ProjectMessage::from(
                    ImportProjectEvent::builder()
                        .scope(bookmark_scope.clone())
                        .stored(event)
                        .build(),
                ));
            }

            // Wait for every imported event id to appear in the
            // bookmark's chronicle.
            let chronicle_for_wait = chronicle.clone();
            let resolve_for_wait = {
                let config = state.config().clone();
                move |hash: &ContentHash| -> Option<LedgerNode> {
                    let db = config.host_db().ok()?;
                    ChronicleStore::new(&db).get(hash)
                }
            };
            state
                .config()
                .fetch
                .eventual(|| async {
                    let root = chronicle_for_wait.root()?;
                    let seen = match root.as_ref() {
                        Some(hash) => Ledger::collect_all_ids(Some(hash), &resolve_for_wait),
                        None => std::collections::HashSet::new(),
                    };
                    Ok::<_, EventError>(if expected_ids.is_subset(&seen) {
                        Some(())
                    } else {
                        None
                    })
                })
                .await?;
        }

        // Store the server's root hash in the checkpoint so we can
        // detect "already up to date" on the next collect.
        let checkpoint = Checkpoint {
            sequence: follow.checkpoint.sequence + events_received,
            cumulative_hash: diff_result.server_root.unwrap_or_default(),
            head: None,
            taken_at: Timestamp::now(),
        };

        FollowService::advance(
            &scope,
            mailbox,
            follow.id,
            checkpoint.clone(),
            events_received,
        )
        .await?;

        Ok(BookmarkResponse::Collected(BookmarkCollectResult {
            follow_id: follow.id,
            events_received,
            checkpoint,
        }))
    }

    /// Remove a follow.
    pub(crate) async fn unfollow(
        state: &ServerState,
        project: &ProjectName,
        request: &UnfollowBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let UnfollowBookmark::V1(unfollowing) = request;
        let name = &unfollowing.name;
        let scope = ComposeScope::new(state.config().clone()).host()?;
        let mailbox = state.mailbox();
        let follow = FollowService::for_bookmark(&scope, project, name)
            .await?
            .ok_or_else(|| BookmarkError::FollowNotFound(name.clone()))?;

        let id = follow.id;
        FollowService::remove(&scope, mailbox, id).await?;

        Ok(BookmarkResponse::Unfollowed(
            BookmarkUnfollowedResponse::builder_v1()
                .follow_id(id)
                .project(project.clone())
                .bookmark(name.clone())
                .build()
                .into(),
        ))
    }

    /// Replay the event log into a specific bookmark's projection DB.
    fn replay_bookmark(
        config: &Config,
        project: &ProjectName,
        bookmark: &BookmarkName,
    ) -> Result<(), BookmarkError> {
        let mut project_config = config.clone();
        project_config.project = project.clone();
        project_config.bookmark = bookmark.clone();
        let db = project_config.bookmark_conn()?;
        let log = EventLog::attached(&db);
        Projections::<ProjectCanon>::project().replay_project(&db, &log)?;
        Ok(())
    }

    /// Create a new bookmark DB and replay events into it.
    ///
    /// Migrates the schema, then replays the event log through
    /// projections so the new bookmark starts with the source's state.
    /// When `event_ids` is non-empty, only matching events are replayed
    /// (scoped fork — used by slice bookmarking).
    fn create_bookmark_db(
        config: &Config,
        project: &ProjectName,
        bookmark: &BookmarkName,
        event_ids: &[EventId],
    ) -> Result<(), BookmarkError> {
        let mut project_config = config.clone();
        project_config.project = project.clone();
        project_config.bookmark = bookmark.clone();

        let bookmarks_dir = project_config.bookmarks_dir();
        project_config
            .platform()
            .ensure_dir(&bookmarks_dir)
            .map_err(|e| BookmarkError::InvalidUri(e.to_string()))?;

        // Open the new bookmark DB with events ATTACHed and replay.
        let db = project_config.bookmark_conn()?;
        let projections = Projections::<ProjectCanon>::project();
        projections.migrate(&db)?;
        let log = EventLog::attached(&db);

        if event_ids.is_empty() {
            // Full replay: all events from the attached log.
            projections.replay_project(&db, &log)?;
        } else {
            // Scoped replay: only events whose IDs are in the filter set.
            let filter: std::collections::HashSet<EventId> = event_ids.iter().copied().collect();
            let all_events = log.load_all()?;
            for event in &all_events {
                if filter.contains(&event.id) {
                    projections.apply_project(&db, event)?;
                }
            }
        }

        Ok(())
    }
}
