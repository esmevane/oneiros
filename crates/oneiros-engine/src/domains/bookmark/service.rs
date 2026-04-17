use crate::*;

pub struct BookmarkService;

impl BookmarkService {
    pub async fn create(
        state: &ServerState,
        brain: &BrainName,
        CreateBookmark { name }: &CreateBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let from = state.canons().active_bookmark(brain)?;
        state.canons().fork_brain(brain, name)?;

        // Create the new bookmark's DB and replay the source events into it.
        Self::create_bookmark_db(state.config(), brain, name)?;

        let forked = BookmarkForked {
            brain: brain.clone(),
            name: name.clone(),
            from,
        };

        state
            .system_context()
            .emit(BookmarkEvents::BookmarkForked(forked.clone()))
            .await?;

        Ok(BookmarkResponse::Forked(forked))
    }

    pub async fn switch(
        state: &ServerState,
        brain: &BrainName,
        SwitchBookmark { name }: &SwitchBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        // With per-bookmark DBs, switch just updates the default.
        // Both bookmark DBs already exist with current projections.
        state.canons().switch_brain(brain, name)?;

        let switched = BookmarkSwitched {
            brain: brain.clone(),
            name: name.clone(),
        };

        state
            .system_context()
            .emit(BookmarkEvents::BookmarkSwitched(switched.clone()))
            .await?;

        Ok(BookmarkResponse::Switched(switched))
    }

    pub async fn merge(
        state: &ServerState,
        brain: &BrainName,
        MergeBookmark { source }: &MergeBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let target = state.canons().active_bookmark(brain)?;
        state.canons().merge_brain(brain, source, &target)?;

        Self::replay_bookmark(state.config(), brain, &target)?;

        let merged = BookmarkMerged {
            brain: brain.clone(),
            source: source.clone(),
            target,
        };

        state
            .system_context()
            .emit(BookmarkEvents::BookmarkMerged(merged.clone()))
            .await?;

        Ok(BookmarkResponse::Merged(merged))
    }

    pub async fn list(
        state: &ServerState,
        brain: &BrainName,
        ListBookmarks { filters }: &ListBookmarks,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let system = state.system_context();
        let listed = BookmarkRepo::new(&system).list(brain, filters).await?;
        Ok(BookmarkResponse::Bookmarks(listed))
    }

    /// Share a bookmark by minting a distribution ticket scoped to it and
    /// composing an `oneiros://` URI that carries the host's address plus
    /// the ticket's link. Delegates to `TicketService::issue` for the
    /// actual ticket minting.
    pub async fn share(
        state: &ServerState,
        brain: &BrainName,
        ShareBookmark { name, actor_id }: &ShareBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let system = state.system_context();
        let all = SearchFilters {
            limit: Limit(usize::MAX),
            offset: Offset(0),
        };

        let bookmarks = BookmarkRepo::new(&system).list(brain, &all).await?;
        let bookmark = bookmarks
            .items
            .iter()
            .find(|b| b.name == *name)
            .ok_or_else(|| BookmarkError::NotFound(name.clone()))?
            .clone();

        let brain_record = BrainRepo::new(&system)
            .get(brain)
            .await?
            .ok_or_else(|| BookmarkError::BrainNotFound(brain.clone()))?;

        let resolved_actor_id = match actor_id {
            Some(id) => *id,
            None => {
                let actors = ActorRepo::new(&system).list(&all).await?;
                actors
                    .items
                    .first()
                    .map(|a| a.id)
                    .ok_or(BookmarkError::NoActor)?
            }
        };

        let target = Ref::bookmark(bookmark.id);
        let ticket =
            TicketService::issue(&system, brain, &brain_record, resolved_actor_id, target).await?;

        let identity = state.host_identity();
        let peer_link = PeerLink::new(identity.address, ticket.link.clone());
        let uri = OneirosUri::Peer(peer_link).to_string();

        system
            .emit(BookmarkEvents::BookmarkShared(BookmarkShared {
                brain: brain.clone(),
                bookmark: name.clone(),
                ticket_id: ticket.id,
                shared_by: resolved_actor_id,
            }))
            .await?;

        Ok(BookmarkResponse::Shared(BookmarkShareResult {
            ticket,
            uri,
        }))
    }

    /// Follow a bookmark via an `oneiros://` or `ref:` URI.
    pub async fn follow(
        state: &ServerState,
        brain: &BrainName,
        FollowBookmark { uri, name }: &FollowBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let system = state.system_context();
        let parsed: OneirosUri = uri
            .parse()
            .map_err(|e: OneirosUriError| BookmarkError::InvalidUri(e.to_string()))?;

        let source = match parsed {
            OneirosUri::Ref(r) => FollowSource::Local(r),
            OneirosUri::Link(link) => FollowSource::Local(link.target),
            OneirosUri::Peer(peer_link) => {
                let key = PeerKey::from_bytes(*peer_link.host.inner().id.as_bytes());
                PeerService::ensure(&system, key, peer_link.host.clone()).await?;
                FollowSource::Peer(peer_link)
            }
        };

        system
            .emit(BookmarkEvents::BookmarkCreated(BookmarkCreated {
                brain: brain.clone(),
                name: name.clone(),
            }))
            .await?;
        state.canons().fork_brain(brain, name)?;

        // Create the new bookmark's DB (empty — collect will populate it).
        Self::create_bookmark_db(state.config(), brain, name)?;

        let follow = FollowService::create(&system, brain.clone(), name.clone(), source).await?;
        Ok(BookmarkResponse::Followed(follow))
    }

    /// Collect from a follow's source.
    pub async fn collect(
        state: &ServerState,
        brain: &BrainName,
        CollectBookmark { name }: &CollectBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let system = state.system_context();
        let follow = FollowService::for_bookmark(&system, brain, name)
            .await?
            .ok_or_else(|| BookmarkError::FollowNotFound(name.clone()))?;

        match follow.source.clone() {
            FollowSource::Local(_) => {
                let checkpoint = Checkpoint::empty();
                FollowService::advance(&system, follow.id, checkpoint.clone(), 0).await?;
                Ok(BookmarkResponse::Collected(BookmarkCollectResult {
                    follow_id: follow.id,
                    events_received: 0,
                    checkpoint,
                }))
            }
            FollowSource::Peer(peer_link) => {
                Self::collect_from_peer(state, brain, &follow, peer_link).await
            }
        }
    }

    /// Collect from a peer via Merkle diff on the chronicle HAMT.
    async fn collect_from_peer(
        state: &ServerState,
        brain: &BrainName,
        follow: &Follow,
        peer_link: PeerLink,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let system = state.system_context();
        let bridge = state.bridge();

        // Get the bookmark's chronicle — this tracks what we've collected.
        let chronicle = state.canons().bookmark_chronicle(brain, &follow.bookmark)?;
        let local_root = chronicle.root()?;

        // Build a local resolver from the system DB's ChronicleStore.
        // Opens its own connection per resolve call — Send-safe across
        // the async diff, and fast with WAL mode (~20 resolves per tree walk).
        let mut brain_config = state.config().clone();
        brain_config.brain = brain.clone();
        {
            let db = state.config().system_db()?;
            ChronicleStore::new(&db).migrate()?;
        }
        let local_resolve = {
            let config = state.config().clone();
            move |hash: &ContentHash| -> Option<LedgerNode> {
                let db = config.system_db().ok()?;
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

        // Phase 2: Fetch missing events and import them.
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
                .map_err(|e: BridgeError| BookmarkError::InvalidUri(e.to_string()))?;

            // Import events to the event log (standalone events.db).
            let events_path = brain_config.events_db_path();
            let events_db = rusqlite::Connection::open(&events_path)?;
            events_db.pragma_update(None, "journal_mode", "wal")?;
            let log = EventLog::new(&events_db);

            // Chronicle objects live in the system DB.
            let system_db = state.config().system_db()?;
            let chronicle_store = ChronicleStore::new(&system_db);
            chronicle_store.migrate()?;

            for event in &events {
                let _ = log.import(event);
                // Record each collected event in the bookmark's chronicle
                // so the next diff can short-circuit on matching roots.
                chronicle.record(
                    event,
                    &chronicle_store.resolver(),
                    &chronicle_store.writer(),
                )?;
            }
        }

        // Phase 3: Replay projections into the follow's bookmark DB.
        Self::replay_bookmark(state.config(), brain, &follow.bookmark)?;

        // Store the server's root hash in the checkpoint so we can
        // detect "already up to date" on the next collect.
        let checkpoint = Checkpoint {
            sequence: follow.checkpoint.sequence + events_received,
            cumulative_hash: diff_result.server_root.unwrap_or_default(),
            head: None,
            taken_at: Timestamp::now(),
        };

        FollowService::advance(&system, follow.id, checkpoint.clone(), events_received).await?;

        Ok(BookmarkResponse::Collected(BookmarkCollectResult {
            follow_id: follow.id,
            events_received,
            checkpoint,
        }))
    }

    /// Remove a follow.
    pub async fn unfollow(
        state: &ServerState,
        brain: &BrainName,
        UnfollowBookmark { name }: &UnfollowBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let system = state.system_context();
        let follow = FollowService::for_bookmark(&system, brain, name)
            .await?
            .ok_or_else(|| BookmarkError::FollowNotFound(name.clone()))?;

        let id = follow.id;
        FollowService::remove(&system, id).await?;

        Ok(BookmarkResponse::Unfollowed(BookmarkUnfollowed {
            follow_id: id,
            brain: brain.clone(),
            bookmark: name.clone(),
        }))
    }

    /// Replay the event log into a specific bookmark's projection DB.
    fn replay_bookmark(
        config: &Config,
        brain: &BrainName,
        bookmark: &BookmarkName,
    ) -> Result<(), BookmarkError> {
        let mut brain_config = config.clone();
        brain_config.brain = brain.clone();
        brain_config.bookmark = bookmark.clone();
        let db = brain_config.bookmark_conn()?;
        let log = EventLog::attached(&db);
        Projections::<BrainCanon>::project().replay_brain(&db, &log)?;
        Ok(())
    }

    /// Create a new bookmark DB and replay events into it.
    ///
    /// Migrates the schema, then replays the event log through
    /// projections so the new bookmark starts with the source's state.
    fn create_bookmark_db(
        config: &Config,
        brain: &BrainName,
        bookmark: &BookmarkName,
    ) -> Result<(), BookmarkError> {
        let mut brain_config = config.clone();
        brain_config.brain = brain.clone();
        brain_config.bookmark = bookmark.clone();

        let bookmarks_dir = brain_config.bookmarks_dir();
        std::fs::create_dir_all(&bookmarks_dir)
            .map_err(|e| BookmarkError::InvalidUri(e.to_string()))?;

        // Open the new bookmark DB with events ATTACHed and replay.
        let db = brain_config.bookmark_conn()?;
        let projections = Projections::<BrainCanon>::project();
        projections.migrate(&db)?;
        let log = EventLog::attached(&db);
        projections.replay_brain(&db, &log)?;

        Ok(())
    }
}
