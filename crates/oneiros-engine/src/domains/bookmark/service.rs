use crate::*;

/// Apply a batch of events received from a peer: write each to the
/// brain's event log, record its id into the target bookmark's
/// chronicle so `bookmark switch` sees it, then rebuild projections.
/// This is a sync helper so the `rusqlite::Connection` never crosses
/// an `await` boundary in the async caller.
fn apply_collected_events(
    config: &Config,
    brain: &BrainName,
    target_chronicle: &Chronicle,
    events: &[StoredEvent],
) -> Result<(), BookmarkError> {
    let mut brain_config = config.clone();
    brain_config.brain = brain.clone();

    // Append each event to the brain's event log and record it in the
    // target bookmark's chronicle. Event ids are globally unique, so
    // import is idempotent on repeat collections.
    {
        let db = brain_config.brain_db()?;
        let log = EventLog::new(&db);
        let chronicle_store = ChronicleStore::new(&db);
        chronicle_store.migrate()?;
        for event in events {
            let _ = log.import(event);
            target_chronicle.record(
                event,
                &chronicle_store.resolver(),
                &chronicle_store.writer(),
            )?;
        }
    }

    // Replay projections from the full log.
    {
        let db = brain_config.brain_db()?;
        let projections = Projections::<BrainCanon>::project();
        projections.migrate(&db)?;
        let all_events = EventLog::new(&db).load_all()?;
        projections.reset(&db)?;
        for event in &all_events {
            projections.apply_frames(&db, event)?;
        }
    }

    Ok(())
}

pub struct BookmarkService;

impl BookmarkService {
    pub async fn create(
        context: &SystemContext,
        canons: &CanonIndex,
        brain: &BrainName,
        CreateBookmark { name }: &CreateBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let from = canons.active_bookmark(brain)?;
        canons.fork_brain(brain, name)?;

        let forked = BookmarkForked {
            brain: brain.clone(),
            name: name.clone(),
            from,
        };

        context
            .emit(BookmarkEvents::BookmarkForked(forked.clone()))
            .await?;

        Ok(BookmarkResponse::Forked(forked))
    }

    pub async fn switch(
        context: &SystemContext,
        canons: &CanonIndex,
        config: &Config,
        brain: &BrainName,
        SwitchBookmark { name }: &SwitchBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let old_chronicle = canons.chronicle(brain)?;
        canons.switch_brain(brain, name)?;
        let new_chronicle = canons.chronicle(brain)?;

        Self::rebuild_projections(config, brain, &old_chronicle, &new_chronicle)?;

        let switched = BookmarkSwitched {
            brain: brain.clone(),
            name: name.clone(),
        };

        context
            .emit(BookmarkEvents::BookmarkSwitched(switched.clone()))
            .await?;

        Ok(BookmarkResponse::Switched(switched))
    }

    pub async fn merge(
        context: &SystemContext,
        canons: &CanonIndex,
        config: &Config,
        brain: &BrainName,
        MergeBookmark { source }: &MergeBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let target = canons.active_bookmark(brain)?;
        canons.merge_brain(brain, source, &target)?;

        Self::rebuild_all_projections(config, brain)?;

        let merged = BookmarkMerged {
            brain: brain.clone(),
            source: source.clone(),
            target,
        };

        context
            .emit(BookmarkEvents::BookmarkMerged(merged.clone()))
            .await?;

        Ok(BookmarkResponse::Merged(merged))
    }

    pub async fn list(
        context: &SystemContext,
        brain: &BrainName,
        ListBookmarks { filters }: &ListBookmarks,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let listed = BookmarkRepo::new(context).list(brain, filters).await?;
        Ok(BookmarkResponse::Bookmarks(listed))
    }

    /// Share a bookmark by minting a distribution ticket scoped to it and
    /// composing an `oneiros://` URI that carries the host's address plus
    /// the ticket's link. Emits `TicketIssued` and `BookmarkShared`.
    ///
    /// The caller supplies the running host's `HostIdentity` — typically
    /// pulled from `ServerState::host_identity()` at the HTTP boundary.
    pub async fn share(
        context: &SystemContext,
        identity: HostIdentity,
        brain: &BrainName,
        ShareBookmark { name, actor_id }: &ShareBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        // Look up the target bookmark to get its stable id.
        let all = SearchFilters {
            limit: Limit(usize::MAX),
            offset: Offset(0),
        };
        let bookmarks = BookmarkRepo::new(context).list(brain, &all).await?;
        let bookmark = bookmarks
            .items
            .iter()
            .find(|b| b.name == *name)
            .ok_or_else(|| BookmarkError::NotFound(name.clone()))?
            .clone();

        // Look up the brain record and the issuing actor to build claims.
        let brain_record = BrainRepo::new(context)
            .get(brain)
            .await?
            .ok_or_else(|| BookmarkError::BrainNotFound(brain.clone()))?;

        // Resolve the issuing actor: explicit if provided, otherwise
        // fall back to the first actor in the system — same convention
        // as `project init`. Single-actor hosts get a zero-arg share.
        let resolved_actor_id = match actor_id {
            Some(id) => *id,
            None => {
                let actors = ActorRepo::new(context).list(&all).await?;
                actors
                    .items
                    .first()
                    .map(|a| a.id)
                    .ok_or(BookmarkError::NoHostIdentity)?
            }
        };

        let actor = ActorRepo::new(context)
            .get(resolved_actor_id)
            .await?
            .ok_or(BookmarkError::ActorNotFound(resolved_actor_id))?;

        // Mint the token + build the ticket with a bookmark target.
        let claims = TokenClaims::builder()
            .brain_id(brain_record.id)
            .tenant_id(actor.tenant_id)
            .actor_id(resolved_actor_id)
            .build();
        let token = Token::issue(claims);
        let target = Ref::bookmark(bookmark.id);
        let link = Link::new(target, token);
        let ticket = Ticket::builder()
            .actor_id(resolved_actor_id)
            .brain_name(brain.clone())
            .brain_id(brain_record.id)
            .link(link.clone())
            .granted_by(resolved_actor_id)
            .build();

        context
            .emit(TicketEvents::TicketIssued(ticket.clone()))
            .await?;

        // Compose the shareable URI: oneiros://<host-address>/link:<payload>
        let peer_link = PeerLink::new(identity.address, link);
        let uri = OneirosUri::Peer(peer_link).to_string();

        // Audit: the bookmark was shared. The URI is not persisted — it's
        // derivable from the ticket + current host identity, and freezing
        // it would capture a stale address snapshot.
        context
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

    /// Follow a bookmark via an `oneiros://` or `ref:` URI. Parses the
    /// URI, ensures any referenced Peer is known, creates a local
    /// bookmark entry with the given name (so `bookmark list` shows it
    /// and `bookmark switch` can reach it), and creates a Follow record
    /// via `FollowService::create`. The Follow's source modality is
    /// determined by the URI variant.
    ///
    /// This operation does not move events. Call `bookmark collect` to
    /// actually fetch and apply events from the source.
    pub async fn follow(
        context: &SystemContext,
        canons: &CanonIndex,
        brain: &BrainName,
        FollowBookmark { uri, name }: &FollowBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let parsed: OneirosUri = uri
            .parse()
            .map_err(|e: OneirosUriError| BookmarkError::InvalidUri(e.to_string()))?;

        let source = match parsed {
            OneirosUri::Ref(r) => FollowSource::Local(r),
            OneirosUri::Peer(peer_link) => {
                // Extract the peer's key from its address and ensure the
                // peer is in our known-peers table.
                let key = PeerKey::from_bytes(*peer_link.host.inner().id.as_bytes());
                PeerService::ensure(context, key, peer_link.host.clone()).await?;
                FollowSource::Peer(peer_link)
            }
            OneirosUri::Link(_) => {
                return Err(BookmarkError::InvalidUri(
                    "link-only URIs have no host segment — nothing to follow".into(),
                ));
            }
        };

        // Create the local bookmark entry so it appears in `bookmark list`
        // and can be switched to, and fork the canon so it has a chronicle
        // of its own into which collected events can be recorded. The
        // fork inherits the active bookmark's current state (empty on a
        // freshly-initialized brain); `bookmark collect` then records the
        // source's events directly into this new chronicle.
        context
            .emit(BookmarkEvents::BookmarkCreated(BookmarkCreated {
                brain: brain.clone(),
                name: name.clone(),
            }))
            .await?;
        canons.fork_brain(brain, name)?;

        let follow = FollowService::create(context, brain.clone(), name.clone(), source).await?;
        Ok(BookmarkResponse::Followed(follow))
    }

    /// Collect events from a follow's source and apply them to the
    /// target bookmark. Dispatches on the Follow's `source` modality:
    /// `Local` reads from the brain's local `CanonIndex` (currently a
    /// no-op; full chronicle-level copy is deferred), `Peer` opens an
    /// iroh connection via the Bridge and runs the sync protocol.
    pub async fn collect(
        context: &SystemContext,
        canons: &CanonIndex,
        bridge: Option<&Bridge>,
        brain: &BrainName,
        CollectBookmark { name }: &CollectBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let follow = FollowService::for_bookmark(context, brain, name)
            .await?
            .ok_or_else(|| BookmarkError::FollowNotFound(name.clone()))?;

        match follow.source.clone() {
            FollowSource::Local(_) => {
                // Local follows don't currently move events — the
                // source bookmark's canon lives in the same brain's
                // CanonIndex and could be queried directly. A cleaner
                // chronicle-level copy is deferred to a follow-up
                // slice; for now we record a no-op collect so the
                // follow lifecycle remains exercisable end-to-end.
                let checkpoint = Checkpoint::empty();
                FollowService::advance(context, follow.id, checkpoint.clone(), 0).await?;
                Ok(BookmarkResponse::Collected(BookmarkCollectResult {
                    follow_id: follow.id,
                    events_received: 0,
                    checkpoint,
                }))
            }
            FollowSource::Peer(peer_link) => {
                Self::collect_from_peer(context, canons, bridge, brain, &follow, peer_link).await
            }
        }
    }

    /// Implementation of `collect` for the Peer source case. Opens an
    /// iroh connection to the peer, sends a `SyncRequest::Pull`, reads
    /// the `SyncResponse`, and applies any returned events to the local
    /// brain's event log and the target bookmark's chronicle.
    async fn collect_from_peer(
        context: &SystemContext,
        canons: &CanonIndex,
        bridge: Option<&Bridge>,
        brain: &BrainName,
        follow: &Follow,
        peer_link: PeerLink,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let bridge = bridge.ok_or(BookmarkError::NoHostIdentity)?;

        let request = SyncRequest::Pull {
            link: peer_link.link.clone(),
            checkpoint: follow.checkpoint.clone(),
        };

        let events = bridge
            .request_events(&peer_link.host, &request)
            .await
            .map_err(|e: BridgeError| BookmarkError::InvalidUri(e.to_string()))?;

        let events_received = events.len() as u64;
        let head = events.last().map(|e| e.id);

        // Resolve the target bookmark's chronicle so collected events
        // are recorded into it — otherwise a subsequent `bookmark
        // switch` would filter them out during projection rebuild.
        let target_chronicle = canons.bookmark_chronicle(brain, &follow.bookmark)?;

        apply_collected_events(&context.config, brain, &target_chronicle, &events)?;

        let checkpoint = Checkpoint {
            sequence: follow.checkpoint.sequence + events_received,
            cumulative_hash: ContentHash::default(),
            head,
            taken_at: Timestamp::now(),
        };

        FollowService::advance(context, follow.id, checkpoint.clone(), events_received).await?;

        Ok(BookmarkResponse::Collected(BookmarkCollectResult {
            follow_id: follow.id,
            events_received,
            checkpoint,
        }))
    }

    /// Remove a follow. Only the remote binding is severed; previously
    /// collected events remain in the local bookmark.
    pub async fn unfollow(
        context: &SystemContext,
        brain: &BrainName,
        UnfollowBookmark { name }: &UnfollowBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let follow = FollowService::for_bookmark(context, brain, name)
            .await?
            .ok_or_else(|| BookmarkError::FollowNotFound(name.clone()))?;

        let id = follow.id;
        FollowService::remove(context, id).await?;

        Ok(BookmarkResponse::Unfollowed(BookmarkUnfollowed {
            follow_id: id,
            brain: brain.clone(),
            bookmark: name.clone(),
        }))
    }

    /// Rebuild all SQLite projections from the event log.
    /// Used after merge when both branches' events should be reflected.
    #[allow(dead_code)]
    fn rebuild_all_projections(config: &Config, brain: &BrainName) -> Result<(), BookmarkError> {
        let mut brain_config = config.clone();
        brain_config.brain = brain.clone();

        let db = brain_config.brain_db()?;
        let projections = Projections::<BrainCanon>::project();
        projections.migrate(&db)?;
        projections.reset(&db)?;

        let all_events = EventLog::new(&db).load_all()?;
        for event in &all_events {
            projections.apply_frames(&db, event)?;
        }

        Ok(())
    }

    /// Rebuild SQLite projections after a bookmark switch.
    ///
    /// Diffs the old and new chronicles to find which events changed,
    /// then resets and replays the new bookmark's events through projections.
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

        if !changes.is_empty() {
            let projections = Projections::<BrainCanon>::project();
            let event_log = EventLog::new(&db);
            let all_events = event_log.load_all()?;

            let new_root = new_chronicle.root()?;
            let new_event_ids: std::collections::HashSet<String> =
                Ledger::collect_all_ids(new_root.as_ref(), &chronicle_store.resolver());

            projections.migrate(&db)?;
            projections.reset(&db)?;

            for event in &all_events {
                if new_event_ids.contains(&event.id.to_string()) {
                    projections.apply_frames(&db, event)?;
                }
            }
        }

        Ok(())
    }
}
