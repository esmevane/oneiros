use crate::*;

/// Internal result from `collect_from_peer_link`.
pub(crate) struct PeerCollectResult {
    pub(crate) count: u64,
    pub(crate) diff_result: DiffResult,
}

pub(crate) struct BookmarkService;

impl BookmarkService {
    /// Ensure a bookmark exists locally. If not, fork the project, create
    /// the bookmark DB, emit a BookmarkForked event, and upsert into the
    /// host store so compose can find it.
    pub(crate) fn ensure_bookmark_exists(
        scope: &Scope<AtHost>,
        canons: &CanonIndex,
        config: &Config,
        mailbox: &Mailbox,
        project: &ProjectName,
        bookmark_name: &BookmarkName,
    ) -> Result<(), BookmarkError> {
        if canons.has_bookmark(project, bookmark_name)? {
            return Ok(());
        }

        canons.fork_project(project, bookmark_name)?;
        Self::create_bookmark_db(config, project, bookmark_name, &[])?;

        let from = canons.active_bookmark(project)?;
        let bookmark = Bookmark::builder()
            .project(project.clone())
            .name(bookmark_name.clone())
            .build();
        let new_event = NewEvent::builder()
            .data(Events::Bookmark(BookmarkEvents::BookmarkForked(
                BookmarkForked::builder_v1()
                    .bookmark(bookmark.clone())
                    .from(from)
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

        let host_db = config.host_db()?;
        BookmarkStore::new(&host_db).upsert(&bookmark)?;
        Ok(())
    }

    pub(crate) async fn create(
        scope: &Scope<AtHost>,
        state: &ServerState,
        project: &ProjectName,
        request: &CreateBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let CreateBookmark::V1(creation) = request;
        let name = &creation.name;
        let from = state.canons().active_bookmark(project)?;

        let mut event_ids = creation.event_ids.clone();

        if let Some(slice_name) = &creation.from_slice {
            let lens_expr = SliceRepo::new(scope)
                .get_lens_expression(slice_name)
                .await?
                .ok_or_else(|| {
                    BookmarkError::InvalidUri(format!("slice not found: {slice_name}"))
                })?;
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
                .scope(scope.clone())
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
        scope: &Scope<AtHost>,
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

        let new_event = NewEvent::builder()
            .data(Events::Bookmark(BookmarkEvents::BookmarkSwitched(
                event.into(),
            )))
            .build();
        state.mailbox().tell(HostMessage::from(
            AppendHostLog::builder()
                .scope(scope.clone())
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
        scope: &Scope<AtHost>,
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

        let new_event = NewEvent::builder()
            .data(Events::Bookmark(BookmarkEvents::BookmarkMerged(
                event.into(),
            )))
            .build();
        state.mailbox().tell(HostMessage::from(
            AppendHostLog::builder()
                .scope(scope.clone())
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
        scope: &Scope<AtHost>,
        state: &ServerState,
        project: &ProjectName,
        request: &ListBookmarks,
    ) -> Result<BookmarkResponse, BookmarkError> {
        match request {
            ListBookmarks::V2(v2) => {
                let Some(ref peer_name) = v2.from else {
                    // V2 without --from: fall through to local list.
                    let listed = BookmarkRepo::new(scope).list(project, &v2.filters).await?;
                    return Ok(BookmarkResponse::Bookmarks(listed));
                };
                let peer = PeerRepo::new(scope)
                    .get_by_name(peer_name)
                    .await?
                    .ok_or_else(|| {
                        BookmarkError::InvalidUri(format!("peer not found: {peer_name}"))
                    })?;
                let ticket = peer.ticket.ok_or_else(|| {
                    BookmarkError::InvalidUri(format!("peer {peer_name} has no ticket"))
                })?;
                let peer_project = peer.project.unwrap_or_else(|| project.clone());

                let list_request = BridgeRequest::BridgeListBookmarks(BridgeListBookmarks {
                    ticket,
                    project: peer_project,
                });
                let response = state
                    .bridge()
                    .send(&peer.address, &list_request)
                    .await
                    .map_err(|e| BookmarkError::InvalidUri(e.to_string()))?;

                match response {
                    BridgeResponse::BridgeBookmarkList(list) => {
                        let bookmarks: Vec<Bookmark> = list
                            .bookmarks
                            .into_iter()
                            .map(|name| {
                                Bookmark::builder()
                                    .project(project.clone())
                                    .name(name)
                                    .build()
                            })
                            .collect();
                        let total = bookmarks.len();
                        Ok(BookmarkResponse::Bookmarks(Listed::new(bookmarks, total)))
                    }
                    BridgeResponse::BridgeDenied(d) => Err(BookmarkError::InvalidUri(d.reason)),
                    _ => Err(BookmarkError::InvalidUri("unexpected response".into())),
                }
            }
            ListBookmarks::V1(v1) => {
                let listed = BookmarkRepo::new(scope).list(project, &v1.filters).await?;
                Ok(BookmarkResponse::Bookmarks(listed))
            }
        }
    }

    /// Share a bookmark by minting a distribution ticket scoped to it and
    /// composing an `oneiros://` URI that carries the host's address plus
    /// the ticket's link. Delegates to `TicketService::issue` for the
    /// actual ticket minting.
    pub(crate) async fn share(
        scope: &Scope<AtHost>,
        state: &ServerState,
        project: &ProjectName,
        request: &ShareBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let ShareBookmark::V1(sharing) = request;
        let name = &sharing.name;
        let actor_id = &sharing.actor_id;
        let mailbox = state.mailbox();
        let all = SearchFilters {
            limit: Limit(usize::MAX),
            offset: Offset(0),
        };

        let bookmarks = BookmarkRepo::new(scope).list(project, &all).await?;
        let bookmark = bookmarks
            .items
            .iter()
            .find(|b| b.name == *name)
            .ok_or_else(|| BookmarkError::NotFound(name.clone()))?
            .clone();

        let project_record = ProjectRepo::new(scope)
            .get(project)
            .await?
            .ok_or_else(|| BookmarkError::ProjectNotFound(project.clone()))?;

        let resolved_actor_id = match actor_id {
            Some(id) => *id,
            None => {
                let actors = ActorRepo::new(scope).list(&all).await?;
                actors
                    .items
                    .first()
                    .map(|a| a.id)
                    .ok_or(BookmarkError::NoActor)?
            }
        };

        let target = Ref::bookmark(bookmark.id);
        let request = IssueTicket::builder_v1()
            .project_name(project.clone())
            .project(project_record)
            .actor_id(resolved_actor_id)
            .target(target)
            .permissions(vec![])
            .build()
            .into();
        let ticket = TicketService::issue(scope, mailbox, &request).await?;

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
                .scope(scope.clone())
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
        scope: &Scope<AtHost>,
        state: &ServerState,
        project: &ProjectName,
        request: &FollowBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let FollowBookmark::V1(following) = request;
        let uri = &following.uri;
        let name = &following.name;
        let mailbox = state.mailbox();
        let parsed: OneirosUri = uri
            .parse()
            .map_err(|e: OneirosUriError| BookmarkError::InvalidUri(e.to_string()))?;

        let source = match parsed {
            OneirosUri::Ref(r) => FollowSource::Local(r),
            OneirosUri::Link(link) => FollowSource::Local(link.target),
            OneirosUri::Peer(peer_link) => {
                let key = PeerKey::from_bytes(*peer_link.host.inner().id.as_bytes());
                PeerService::ensure(scope, mailbox, key, peer_link.host.clone()).await?;
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
            FollowService::create(scope, mailbox, project.clone(), name.clone(), source).await?;
        Ok(BookmarkResponse::Followed(follow))
    }

    /// Collect events into a bookmark — from a follow source or directly
    /// from a peer.
    pub(crate) async fn collect(
        scope: &Scope<AtHost>,
        state: &ServerState,
        project: &ProjectName,
        request: &CollectBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        match request {
            CollectBookmark::V2(v2) => {
                let Some(ref peer_name) = v2.from else {
                    // V2 without --from: fall through to follow-based collect.
                    let mailbox = state.mailbox();
                    let follow = FollowService::for_bookmark(scope, project, &v2.name)
                        .await?
                        .ok_or_else(|| BookmarkError::FollowNotFound(v2.name.clone()))?;

                    return match follow.source.clone() {
                        FollowSource::Local(_) => {
                            let checkpoint = Checkpoint::empty();
                            FollowService::advance(
                                scope,
                                mailbox,
                                follow.id,
                                checkpoint.clone(),
                                0,
                            )
                            .await?;
                            Ok(BookmarkResponse::Collected(BookmarkCollectResult {
                                follow_id: Some(follow.id),
                                events_received: 0,
                                checkpoint,
                            }))
                        }
                        FollowSource::Peer(peer_link) => {
                            Self::collect_from_peer(scope, state, project, &follow, peer_link).await
                        }
                    };
                };
                return Self::collect_from_remote(
                    scope,
                    state,
                    project,
                    &v2.name,
                    peer_name,
                    v2.as_name.as_ref(),
                )
                .await;
            }
            CollectBookmark::V1(v1) => {
                let mailbox = state.mailbox();
                let follow = FollowService::for_bookmark(scope, project, &v1.name)
                    .await?
                    .ok_or_else(|| BookmarkError::FollowNotFound(v1.name.clone()))?;

                match follow.source.clone() {
                    FollowSource::Local(_) => {
                        let checkpoint = Checkpoint::empty();
                        FollowService::advance(scope, mailbox, follow.id, checkpoint.clone(), 0)
                            .await?;
                        Ok(BookmarkResponse::Collected(BookmarkCollectResult {
                            follow_id: Some(follow.id),
                            events_received: 0,
                            checkpoint,
                        }))
                    }
                    FollowSource::Peer(peer_link) => {
                        Self::collect_from_peer(scope, state, project, &follow, peer_link).await
                    }
                }
            }
        }
    }

    /// Collect directly from a peer, using the chronicle diff
    /// protocol. Creates the local bookmark if it doesn't exist.
    async fn collect_from_remote(
        scope: &Scope<AtHost>,
        state: &ServerState,
        project: &ProjectName,
        peer_bookmark_name: &BookmarkName,
        peer_name: &PeerName,
        as_name: Option<&BookmarkName>,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let peer = PeerRepo::new(scope)
            .get_by_name(peer_name)
            .await?
            .ok_or_else(|| BookmarkError::InvalidUri(format!("peer not found: {}", peer_name)))?;

        let local_name = as_name.unwrap_or(peer_bookmark_name);

        // Ensure the bookmark exists locally.
        Self::ensure_bookmark_exists(
            scope,
            state.canons(),
            state.config(),
            state.mailbox(),
            project,
            local_name,
        )?;

        let ticket = peer.ticket.ok_or_else(|| {
            BookmarkError::InvalidUri(format!("peer {} has no ticket", peer_name))
        })?;
        let peer_link = PeerLink::new(peer.address, ticket);
        let result = Self::collect_from_peer_link(
            state.mailbox(),
            state.bridge(),
            state.canons(),
            state.config(),
            project,
            local_name,
            peer_link,
        )
        .await?;

        let checkpoint = Checkpoint {
            sequence: result.count,
            cumulative_hash: result.diff_result.server_root.unwrap_or_default(),
            head: None,
            taken_at: Timestamp::now(),
        };

        Ok(BookmarkResponse::Collected(BookmarkCollectResult {
            follow_id: None,
            events_received: result.count,
            checkpoint,
        }))
    }

    /// Collect from a peer via Merkle diff on the chronicle HAMT.
    ///
    /// The local-side ingestion flows through the bus as `Import`
    /// messages. The inbound actor handles the events.db insert
    /// (insert-or-ignore by id), then forwards `Stored` to the
    /// bookmark + chronicle children — which project and record
    /// without distinction from locally-emitted events.
    async fn collect_from_peer(
        scope: &Scope<AtHost>,
        state: &ServerState,
        project: &ProjectName,
        follow: &Follow,
        peer_link: PeerLink,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let mailbox = state.mailbox();

        let result = Self::collect_from_peer_link(
            state.mailbox(),
            state.bridge(),
            state.canons(),
            state.config(),
            project,
            &follow.bookmark,
            peer_link,
        )
        .await?;
        let events_received = result.count;

        let checkpoint = Checkpoint {
            sequence: follow.checkpoint.sequence + events_received,
            cumulative_hash: result.diff_result.server_root.unwrap_or_default(),
            head: None,
            taken_at: Timestamp::now(),
        };

        FollowService::advance(
            scope,
            mailbox,
            follow.id,
            checkpoint.clone(),
            events_received,
        )
        .await?;

        Ok(BookmarkResponse::Collected(BookmarkCollectResult {
            follow_id: Some(follow.id),
            events_received,
            checkpoint,
        }))
    }

    /// Collect events from a peer link into a named bookmark, without
    /// requiring a Follow record. Used by both the existing collect flow
    /// and the submit handshake.
    ///
    /// The caller must ensure the bookmark already exists on this host.
    pub(crate) async fn collect_from_peer_link(
        mailbox: &Mailbox,
        bridge: &Bridge,
        canons: &CanonIndex,
        config: &Config,
        project: &ProjectName,
        bookmark_name: &BookmarkName,
        peer_link: PeerLink,
    ) -> Result<PeerCollectResult, BookmarkError> {
        let chronicle = canons.bookmark_chronicle(project, bookmark_name)?;
        let local_root = chronicle.root()?;

        {
            let db = config.host_db()?;
            ChronicleStore::new(&db).migrate()?;
        }
        let local_resolve = {
            let config = config.clone();
            move |hash: &ContentHash| -> Option<LedgerNode> {
                let db = config.host_db().ok()?;
                ChronicleStore::new(&db).get(hash)
            }
        };

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

            let bookmark_scope = ComposeScope::new(config.clone())
                .bookmark(project.clone(), bookmark_name.clone())?;

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

            let chronicle_for_wait = chronicle.clone();
            let resolve_for_wait = {
                let config = config.clone();
                move |hash: &ContentHash| -> Option<LedgerNode> {
                    let db = config.host_db().ok()?;
                    ChronicleStore::new(&db).get(hash)
                }
            };
            config
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

        Ok(PeerCollectResult {
            count: events_received,
            diff_result,
        })
    }

    /// Remove a follow.
    pub(crate) async fn unfollow(
        scope: &Scope<AtHost>,
        state: &ServerState,
        project: &ProjectName,
        request: &UnfollowBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let UnfollowBookmark::V1(unfollowing) = request;
        let name = &unfollowing.name;
        let mailbox = state.mailbox();
        let follow = FollowService::for_bookmark(scope, project, name)
            .await?
            .ok_or_else(|| BookmarkError::FollowNotFound(name.clone()))?;

        let id = follow.id;
        FollowService::remove(scope, mailbox, id).await?;

        Ok(BookmarkResponse::Unfollowed(
            BookmarkUnfollowedResponse::builder_v1()
                .follow_id(id)
                .project(project.clone())
                .bookmark(name.clone())
                .build()
                .into(),
        ))
    }

    /// Submit a bookmark to a peer host.
    pub(crate) async fn submit(
        scope: &Scope<AtHost>,
        state: &ServerState,
        project: &ProjectName,
        request: &SubmitBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let req = request
            .current()
            .map_err(|e| BookmarkError::InvalidUri(format!("version upcast failed: {e}")))?;

        let peer = PeerRepo::new(scope)
            .get_by_name(&req.peer)
            .await?
            .ok_or_else(|| BookmarkError::InvalidUri(format!("peer not found: {}", req.peer)))?;

        let peer_name = req.as_name.unwrap_or_else(|| req.name.clone());

        // Share the local bookmark to get a peer link.
        let share_result = match Self::share(
            scope,
            state,
            project,
            &ShareBookmark::builder_v1()
                .name(req.name.clone())
                .build()
                .into(),
        )
        .await?
        {
            BookmarkResponse::Shared(result) => result,
            _ => return Err(BookmarkError::InvalidUri("share failed".into())),
        };

        let ticket = peer
            .ticket
            .ok_or_else(|| BookmarkError::InvalidUri(format!("peer {} has no ticket", req.peer)))?;

        let submit_request = BridgeRequest::BridgeSubmitBookmark(BridgeSubmitBookmark {
            ticket,
            bookmark: PeerLink::new(state.host_identity().address, share_result.ticket.link),
            bookmark_name: peer_name.clone(),
        });

        let response = state
            .bridge()
            .send(&peer.address, &submit_request)
            .await
            .map_err(|e| BookmarkError::InvalidUri(e.to_string()))?;

        let (accepted, reason) = match response {
            BridgeResponse::BridgeSubmitAccepted => (true, None),
            BridgeResponse::BridgeSubmitRejected(d) => (false, Some(d.reason)),
            BridgeResponse::BridgeDenied(d) => (false, Some(d.reason)),
            _ => (false, Some("unexpected response".into())),
        };

        Ok(BookmarkResponse::Submitted(BookmarkSubmitResult {
            accepted,
            bookmark_name: peer_name,
            reason,
        }))
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
    pub(crate) fn create_bookmark_db(
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
