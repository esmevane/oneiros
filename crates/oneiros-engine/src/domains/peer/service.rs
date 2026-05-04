use crate::*;

pub struct PeerService;

fn peer_to_added_v1(peer: Peer) -> PeerAddedResponseV1 {
    PeerAddedResponseV1 {
        id: peer.id,
        key: peer.key,
        address: peer.address,
        name: peer.name,
        created_at: peer.created_at,
    }
}

fn peer_to_found_v1(peer: Peer) -> PeerFoundResponseV1 {
    PeerFoundResponseV1 {
        id: peer.id,
        key: peer.key,
        address: peer.address,
        name: peer.name,
        created_at: peer.created_at,
    }
}

impl PeerService {
    pub async fn add(context: &HostLog, request: &AddPeer) -> Result<PeerResponse, PeerError> {
        let AddPeer::V1(add) = request;
        let parsed: PeerAddress = add
            .address
            .parse()
            .map_err(|e: PeerAddressError| PeerError::InvalidAddress(e.to_string()))?;
        let endpoint_id = parsed.inner().id;
        let key = PeerKey::from_bytes(*endpoint_id.as_bytes());

        let peer_name = match &add.name {
            Some(name) if !name.is_empty() => PeerName::new(name),
            _ => default_peer_name(&key),
        };

        let peer = Peer::builder()
            .key(key)
            .address(parsed)
            .name(peer_name)
            .build();

        let event = PeerAdded::builder_v1()
            .id(peer.id)
            .key(peer.key)
            .address(peer.address.clone())
            .name(peer.name.clone())
            .created_at(peer.created_at)
            .build();

        context.emit(PeerEvents::PeerAdded(event.into())).await?;
        Ok(PeerResponse::Added(PeerAddedResponse::V1(
            peer_to_added_v1(peer),
        )))
    }

    pub async fn get(context: &HostLog, request: &GetPeer) -> Result<PeerResponse, PeerError> {
        let GetPeer::V1(get) = request;
        let id = get.key.resolve()?;
        let peer = PeerRepo::new(context.scope()?)
            .fetch(id)
            .await?
            .ok_or(PeerError::NotFound(id))?;
        Ok(PeerResponse::Found(PeerFoundResponse::V1(
            peer_to_found_v1(peer),
        )))
    }

    pub async fn list(context: &HostLog, request: &ListPeers) -> Result<PeerResponse, PeerError> {
        let ListPeers::V1(listing) = request;
        let listed = PeerRepo::new(context.scope()?)
            .list(&listing.filters)
            .await?;
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

    /// Ensure a peer with the given key is known. If one already exists,
    /// return it without emitting an event. Otherwise add it and return
    /// the newly-created record.
    pub async fn ensure(
        context: &HostLog,
        key: PeerKey,
        address: PeerAddress,
    ) -> Result<Peer, PeerError> {
        let all = SearchFilters {
            limit: Limit(usize::MAX),
            offset: Offset(0),
        };
        let listed = PeerRepo::new(context.scope()?).list(&all).await?;
        if let Some(existing) = listed.items.iter().find(|p| p.key == key) {
            return Ok(existing.clone());
        }

        let name = default_peer_name(&key);
        let peer = Peer::builder().key(key).address(address).name(name).build();

        let event = PeerAdded::builder_v1()
            .id(peer.id)
            .key(peer.key)
            .address(peer.address.clone())
            .name(peer.name.clone())
            .created_at(peer.created_at)
            .build();

        context.emit(PeerEvents::PeerAdded(event.into())).await?;

        Ok(peer)
    }

    pub async fn remove(
        context: &HostLog,
        request: &RemovePeer,
    ) -> Result<PeerResponse, PeerError> {
        let RemovePeer::V1(remove) = request;
        let existing = PeerRepo::new(context.scope()?)
            .get(remove.id)
            .await?
            .ok_or(PeerError::NotFound(remove.id))?;

        context
            .emit(PeerEvents::PeerRemoved(
                PeerRemoved::builder_v1().id(existing.id).build().into(),
            ))
            .await?;

        Ok(PeerResponse::Removed(
            PeerRemovedResponse::builder_v1()
                .id(remove.id)
                .build()
                .into(),
        ))
    }
}

/// Default peer name derived from the first 6 hex digits of the key.
fn default_peer_name(key: &PeerKey) -> PeerName {
    let hex = key.to_string();
    let prefix: String = hex.chars().take(6).collect();
    PeerName::new(format!("peer-{prefix}"))
}
