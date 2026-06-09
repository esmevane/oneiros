use crate::*;

pub(crate) struct PeerService;

fn peer_to_added_v1(peer: Peer) -> PeerAddedResponseV1 {
    PeerAddedResponseV1 {
        id: peer.id,
        key: peer.key,
        address: peer.address,
        name: peer.name,
        kind: peer.kind,
        ticket: peer.ticket,
        project: peer.project,
        created_at: peer.created_at,
    }
}

fn peer_to_found_v1(peer: Peer) -> PeerFoundResponseV1 {
    PeerFoundResponseV1 {
        id: peer.id,
        key: peer.key,
        address: peer.address,
        name: peer.name,
        kind: peer.kind,
        ticket: peer.ticket,
        project: peer.project,
        created_at: peer.created_at,
    }
}

impl PeerService {
    pub(crate) async fn add(
        scope: &Scope<AtHost>,
        mailbox: &Mailbox,
        request: &AddPeer,
    ) -> Result<PeerResponse, PeerError> {
        let AddPeer::V1(add) = request;

        let (parsed_addr, kind, ticket, peer_project, peer_name) =
            if let Ok(uri) = add.address.parse::<OneirosUri>() {
                match uri {
                    OneirosUri::Peer(pl) => {
                        let addr = pl.host;
                        let name = match &add.name {
                            Some(n) if !n.is_empty() => PeerName::new(n),
                            _ => {
                                let key = PeerKey::from_bytes(*addr.inner().id.as_bytes());
                                default_peer_name(&key)
                            }
                        };
                        let pticket = pl.link.clone();
                        let kind = match &pticket.target {
                            Ref::V0(Resource::Project(_)) => PeerKind::Project,
                            Ref::V0(Resource::Bookmark(_)) => PeerKind::Bookmark,
                            _ => return Err(PeerError::InvalidRef),
                        };
                        let project = match &pticket.target {
                            Ref::V0(Resource::Project(_)) => Some(scope.config().project.clone()),
                            _ => None,
                        };
                        (addr, kind, Some(pticket), project, name)
                    }
                    _ => {
                        return Err(PeerError::InvalidAddress);
                    }
                }
            } else {
                let parsed: PeerAddress = add.address.parse()?;
                let endpt = parsed.inner().id;
                let key = PeerKey::from_bytes(*endpt.as_bytes());
                let name = match &add.name {
                    Some(n) if !n.is_empty() => PeerName::new(n),
                    _ => default_peer_name(&key),
                };
                (parsed, PeerKind::Bookmark, None, None, name)
            };

        let peer = Peer::builder()
            .key(PeerKey::from_bytes(*parsed_addr.inner().id.as_bytes()))
            .address(parsed_addr)
            .name(peer_name)
            .kind(kind)
            .maybe_ticket(ticket)
            .maybe_project(peer_project)
            .build();

        let event = PeerAdded::builder_v1()
            .id(peer.id)
            .key(peer.key)
            .address(peer.address.clone())
            .name(peer.name.clone())
            .kind(peer.kind)
            .maybe_ticket(peer.ticket.clone())
            .maybe_project(peer.project.clone())
            .created_at(peer.created_at)
            .build();

        let new_event = NewEvent::builder()
            .data(Events::Peer(PeerEvents::PeerAdded(event.into())))
            .build();

        mailbox.tell(HostMessage::from(
            AppendHostLog::builder()
                .scope(scope.clone())
                .event(new_event)
                .build(),
        ));

        Ok(PeerResponse::Added(PeerAddedResponse::V1(
            peer_to_added_v1(peer),
        )))
    }

    pub(crate) async fn get(
        scope: &Scope<AtHost>,
        request: &GetPeer,
    ) -> Result<PeerResponse, PeerError> {
        let GetPeer::V1(get) = request;
        let id = get.key.resolve()?;
        let peer = PeerRepo::new(scope)
            .fetch(id)
            .await?
            .ok_or(PeerError::NotFound(id))?;
        Ok(PeerResponse::Found(PeerFoundResponse::V1(
            peer_to_found_v1(peer),
        )))
    }

    pub(crate) async fn list(
        scope: &Scope<AtHost>,
        request: &ListPeers,
    ) -> Result<PeerResponse, PeerError> {
        let ListPeers::V1(listing) = request;
        let listed = PeerRepo::new(scope).list(&listing.filters).await?;
        let total = listed.total;
        let items: Vec<PeerFoundResponseV1> =
            listed.items.into_iter().map(peer_to_found_v1).collect();
        Ok(PeerResponse::Listed(
            PeersResponse::builder_v1()
                .items(items)
                .total(total)
                .build()
                .into(),
        ))
    }

    pub(crate) async fn ensure(
        scope: &Scope<AtHost>,
        mailbox: &Mailbox,
        key: PeerKey,
        address: PeerAddress,
    ) -> Result<Peer, PeerError> {
        let all = SearchFilters {
            limit: Limit(usize::MAX),
            offset: Offset(0),
        };
        let listed = PeerRepo::new(scope).list(&all).await?;
        if let Some(existing) = listed.items.iter().find(|p| p.key == key) {
            return Ok(existing.clone());
        }

        let name = default_peer_name(&key);
        let peer = Peer::builder()
            .key(key)
            .address(address)
            .name(name)
            .kind(PeerKind::Bookmark)
            .build();

        let event = PeerAdded::builder_v1()
            .id(peer.id)
            .key(peer.key)
            .address(peer.address.clone())
            .name(peer.name.clone())
            .kind(peer.kind)
            .maybe_ticket(peer.ticket.clone())
            .maybe_project(peer.project.clone())
            .created_at(peer.created_at)
            .build();

        let new_event = NewEvent::builder()
            .data(Events::Peer(PeerEvents::PeerAdded(event.into())))
            .build();

        mailbox.tell(HostMessage::from(
            AppendHostLog::builder()
                .scope(scope.clone())
                .event(new_event)
                .build(),
        ));

        Ok(peer)
    }

    pub(crate) async fn remove(
        scope: &Scope<AtHost>,
        mailbox: &Mailbox,
        request: &RemovePeer,
    ) -> Result<PeerResponse, PeerError> {
        let RemovePeer::V1(remove) = request;
        let existing = PeerRepo::new(scope)
            .get(remove.id)
            .await?
            .ok_or(PeerError::NotFound(remove.id))?;

        let new_event = NewEvent::builder()
            .data(Events::Peer(PeerEvents::PeerRemoved(
                PeerRemoved::builder_v1().id(existing.id).build().into(),
            )))
            .build();

        mailbox.tell(HostMessage::from(
            AppendHostLog::builder()
                .scope(scope.clone())
                .event(new_event)
                .build(),
        ));

        Ok(PeerResponse::Removed(
            PeerRemovedResponse::builder_v1()
                .id(remove.id)
                .build()
                .into(),
        ))
    }
}

fn default_peer_name(key: &PeerKey) -> PeerName {
    let hex = key.to_string();
    let prefix: String = hex.chars().take(6).collect();
    PeerName::new(format!("peer-{prefix}"))
}
