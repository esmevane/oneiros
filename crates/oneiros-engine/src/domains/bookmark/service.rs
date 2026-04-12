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
        let old_chronicle = state.canons().chronicle(brain)?;
        state.canons().switch_brain(brain, name)?;
        let new_chronicle = state.canons().chronicle(brain)?;

        Self::rebuild_projections(state.config(), brain, &old_chronicle, &new_chronicle)?;

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

        Self::replay_brain_projections(state.config(), brain)?;

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

        let follow = FollowService::create(&system, brain.clone(), name.clone(), source).await?;
        Ok(BookmarkResponse::Followed(follow))
    }

    /// Collect from a follow's source via CRDT conference.
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

    /// Collect from a peer via CRDT conference.
    async fn collect_from_peer(
        state: &ServerState,
        brain: &BrainName,
        follow: &Follow,
        peer_link: PeerLink,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let system = state.system_context();
        let bridge = state.bridge();
        let canon = state.canons().brain(brain)?;

        // Phase 1: Confer — CRDT delta exchange.
        let version_vector = canon.version_vector();
        let confer_request = SyncRequest::Confer {
            link: peer_link.link.clone(),
            version_vector,
        };

        let updates = bridge
            .confer(&peer_link.host, &confer_request)
            .await
            .map_err(|e: BridgeError| BookmarkError::InvalidUri(e.to_string()))?;

        if let Some(bytes) = &updates {
            canon.import_updates(bytes)?;
        }

        // Phase 2: Derive manifest — which events does the canon have
        // that the local event log doesn't?
        let canon_event_ids = canon.event_ids();
        let mut brain_config = state.config().clone();
        brain_config.brain = brain.clone();
        let local_event_ids: std::collections::HashSet<String> = {
            let db = brain_config.brain_db()?;
            EventLog::new(&db)
                .load_all()?
                .into_iter()
                .map(|e| e.id.to_string())
                .collect()
        };

        let missing: Vec<String> = canon_event_ids
            .difference(&local_event_ids)
            .cloned()
            .collect();

        let events_received = missing.len() as u64;

        // Phase 3: Fetch missing events from the peer and import them.
        if !missing.is_empty() {
            let fetch_request = SyncRequest::FetchEvents {
                link: peer_link.link.clone(),
                event_ids: missing,
            };

            let events = bridge
                .fetch_events(&peer_link.host, &fetch_request)
                .await
                .map_err(|e: BridgeError| BookmarkError::InvalidUri(e.to_string()))?;

            let db = brain_config.brain_db()?;
            let log = EventLog::new(&db);
            for event in &events {
                let _ = log.import(event);
            }
        }

        // Phase 4: Replay projections from the updated event log.
        Self::replay_brain_projections(state.config(), brain)?;

        let checkpoint = Checkpoint {
            sequence: follow.checkpoint.sequence + events_received,
            cumulative_hash: ContentHash::default(),
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

    fn replay_brain_projections(config: &Config, brain: &BrainName) -> Result<(), BookmarkError> {
        let mut brain_config = config.clone();
        brain_config.brain = brain.clone();
        let db = brain_config.brain_db()?;
        Projections::<BrainCanon>::project().replay_brain(&db)?;
        Ok(())
    }

    fn rebuild_projections(
        config: &Config,
        brain: &BrainName,
        old_chronicle: &Chronicle,
        new_chronicle: &Chronicle,
    ) -> Result<(), BookmarkError> {
        let mut brain_config = config.clone();
        brain_config.brain = brain.clone();
        let db = brain_config.brain_db()?;

        let chronicle_store = ChronicleStore::new(&db);
        chronicle_store.migrate()?;

        let changes = old_chronicle.diff(new_chronicle, &chronicle_store.resolver())?;
        if changes.is_empty() {
            return Ok(());
        }

        let new_root = new_chronicle.root()?;
        let new_event_ids: std::collections::HashSet<String> =
            Ledger::collect_all_ids(new_root.as_ref(), &chronicle_store.resolver());

        let projections = Projections::<BrainCanon>::project();
        let all_events = EventLog::new(&db).load_all()?;

        projections.migrate(&db)?;
        projections.reset(&db)?;

        for event in &all_events {
            if new_event_ids.contains(&event.id.to_string()) {
                projections.apply_frames(&db, event)?;
            }
        }

        Ok(())
    }
}
