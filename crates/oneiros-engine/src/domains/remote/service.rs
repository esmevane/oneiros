use crate::*;

pub(crate) struct RemoteService;

impl RemoteService {
    pub(crate) async fn add(
        state: &ServerState,
        request: &AddRemote,
    ) -> Result<RemoteResponse, RemoteError> {
        let AddRemote::V1(add) = request;
        let scope = ComposeScope::new(state.config().clone()).host()?;

        // If a remote with this name already exists, remove it first (upsert).
        if let Some(existing) = RemoteRepo::new(&scope).get_by_name(&add.name).await? {
            let remove_event = NewEvent::builder()
                .data(Events::Remote(RemoteEvents::RemoteRemoved(
                    RemoteRemoved::builder_v1().id(existing.id).build().into(),
                )))
                .build();
            state.mailbox().tell(HostMessage::from(
                AppendHostLog::builder()
                    .scope(scope.clone())
                    .event(remove_event)
                    .build(),
            ));
        }

        // Parse the ticket URI.
        let uri: OneirosUri = add
            .ticket
            .parse()
            .map_err(|_| RemoteError::InvalidTicket(add.ticket.clone()))?;
        let peer_link = match uri {
            OneirosUri::Peer(pl) => pl,
            _ => {
                return Err(RemoteError::InvalidTicket(
                    "URI must be an oneiros:// link".into(),
                ));
            }
        };

        // Validate the ticket by connecting to the peer and listing bookmarks.
        let list_request = BridgeRequest::BridgeListBookmarks(BridgeListBookmarks {
            ticket: peer_link.link.clone(),
            project: state.config().project.clone(),
        });
        let response = state
            .bridge()
            .send(&peer_link.host, &list_request)
            .await
            .map_err(|e| RemoteError::ConnectionFailed(e.to_string()))?;

        if matches!(response, BridgeResponse::BridgeDenied(_)) {
            return Err(RemoteError::InvalidTicket(
                "ticket rejected by remote".into(),
            ));
        }

        // Persist the remote.
        let remote = Remote::builder()
            .name(add.name.clone())
            .address(peer_link.host)
            .ticket(peer_link.link)
            .project(state.config().project.clone())
            .build();
        let id = remote.id;

        let new_event = NewEvent::builder()
            .data(Events::Remote(RemoteEvents::RemoteAdded(
                RemoteAdded::builder_v1().remote(remote).build().into(),
            )))
            .build();
        state.mailbox().tell(HostMessage::from(
            AppendHostLog::builder()
                .scope(scope)
                .event(new_event)
                .build(),
        ));

        let stored = RemoteRepo::new(&ComposeScope::new(state.config().clone()).host()?)
            .fetch(&id)
            .await?
            .ok_or(RemoteError::NotFound(add.name.clone()))?;

        Ok(RemoteResponse::Added(
            RemoteAddedResponse::builder_v1()
                .remote(stored)
                .build()
                .into(),
        ))
    }

    pub(crate) async fn list(
        scope: &Scope<AtHost>,
        request: &ListRemotes,
    ) -> Result<RemoteResponse, RemoteError> {
        let ListRemotes::V1(listing) = request;
        let listed = RemoteRepo::new(scope).list(&listing.filters).await?;
        Ok(RemoteResponse::Listed(
            RemotesResponse::builder_v1()
                .items(listed.items)
                .total(listed.total)
                .build()
                .into(),
        ))
    }

    pub(crate) async fn remove(
        scope: &Scope<AtHost>,
        mailbox: &Mailbox,
        request: &RemoveRemote,
    ) -> Result<RemoteResponse, RemoteError> {
        let RemoveRemote::V1(remove) = request;
        let remote = RemoteRepo::new(scope)
            .get_by_name(&remove.name)
            .await?
            .ok_or_else(|| RemoteError::NotFound(remove.name.clone()))?;

        let new_event = NewEvent::builder()
            .data(Events::Remote(RemoteEvents::RemoteRemoved(
                RemoteRemoved::builder_v1().id(remote.id).build().into(),
            )))
            .build();
        mailbox.tell(HostMessage::from(
            AppendHostLog::builder()
                .scope(scope.clone())
                .event(new_event)
                .build(),
        ));

        Ok(RemoteResponse::Removed(
            RemoteRemovedResponse::builder_v1()
                .id(remote.id)
                .name(remote.name)
                .build()
                .into(),
        ))
    }

    /// List bookmarks on a remote by connecting and sending BridgeListBookmarks.
    pub(crate) async fn bookmarks(
        state: &ServerState,
        name: &RemoteName,
    ) -> Result<RemoteResponse, RemoteError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        let remote = RemoteRepo::new(&scope)
            .get_by_name(name)
            .await?
            .ok_or_else(|| RemoteError::NotFound(name.clone()))?;

        let list_request = BridgeRequest::BridgeListBookmarks(BridgeListBookmarks {
            ticket: remote.ticket.clone(),
            project: remote.project.clone(),
        });
        let response = state
            .bridge()
            .send(&remote.address, &list_request)
            .await
            .map_err(|e| RemoteError::ConnectionFailed(e.to_string()))?;

        match response {
            BridgeResponse::BridgeBookmarkList(list) => Ok(RemoteResponse::Bookmarks(
                RemoteBookmarkListResponse::builder_v1()
                    .bookmarks(list.bookmarks)
                    .build()
                    .into(),
            )),
            BridgeResponse::BridgeDenied(d) => Err(RemoteError::InvalidTicket(d.reason)),
            _ => Err(RemoteError::ConnectionFailed("unexpected response".into())),
        }
    }

    /// Share a project by issuing (or reusing) a project-scoped ticket.
    pub(crate) async fn share(
        state: &ServerState,
        request: &ShareRemote,
    ) -> Result<RemoteResponse, RemoteError> {
        let ShareRemote::V1(req) = request;
        let scope = ComposeScope::new(state.config().clone()).host()?;

        // Get or create the project.
        let project = ProjectRepo::new(&scope)
            .get(&req.project)
            .await?
            .ok_or_else(|| {
                RemoteError::ConnectionFailed(format!("project not found: {}", req.project))
            })?;

        // Issue a ticket for the project.
        let target = Ref::project(project.id);
        let all = SearchFilters {
            limit: Limit(usize::MAX),
            offset: Offset(0),
        };
        let actor = ActorRepo::new(&scope)
            .list(&all)
            .await?
            .items
            .into_iter()
            .next()
            .ok_or_else(|| RemoteError::ConnectionFailed("no actors found".into()))?;

        let ticket = TicketService::issue(
            &scope,
            state.mailbox(),
            &req.project,
            &project,
            actor.id,
            target,
            vec![
                Permission::from(PermissionV1 {
                    operation: PermissionOp::BookmarkPush,
                }),
                Permission::from(PermissionV1 {
                    operation: PermissionOp::BookmarkPull,
                }),
                Permission::from(PermissionV1 {
                    operation: PermissionOp::BookmarkList,
                }),
            ],
        )
        .await?;

        let peer_link = PeerLink::new(state.host_identity().address, ticket.link.clone());
        let uri = OneirosUri::Peer(peer_link).to_string();

        Ok(RemoteResponse::Shared(
            RemoteSharedResponse::builder_v1()
                .ticket(ticket)
                .uri(uri)
                .build()
                .into(),
        ))
    }
}
