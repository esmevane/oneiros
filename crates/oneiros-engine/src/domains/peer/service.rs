use crate::*;

pub struct PeerService;

impl PeerService {
    /// Add a peer by its base64url-encoded address. Extracts the key from
    /// the address, generates a default name if none is supplied, and
    /// emits `PeerAdded`.
    pub async fn add(
        context: &SystemContext,
        AddPeer { address, name }: &AddPeer,
    ) -> Result<PeerResponse, PeerError> {
        let parsed: PeerAddress = address
            .parse()
            .map_err(|e: PeerAddressError| PeerError::InvalidAddress(e.to_string()))?;
        let endpoint_id = parsed.inner().id;
        let key = PeerKey::from_bytes(*endpoint_id.as_bytes());

        let peer_name = match name {
            Some(name) if !name.is_empty() => PeerName::new(name),
            _ => default_peer_name(&key),
        };

        let peer = Peer::builder()
            .key(key)
            .address(parsed)
            .name(peer_name)
            .build();

        context.emit(PeerEvents::PeerAdded(peer.clone())).await?;
        let ref_token = RefToken::new(Ref::peer(peer.id));
        Ok(PeerResponse::Added(
            Response::new(peer).with_ref_token(ref_token),
        ))
    }

    pub async fn get(
        context: &SystemContext,
        selector: &GetPeer,
    ) -> Result<PeerResponse, PeerError> {
        let id = selector.key.resolve()?;
        let peer = PeerRepo::new(context)
            .get(id)
            .await?
            .ok_or(PeerError::NotFound(id))?;
        let ref_token = RefToken::new(Ref::peer(peer.id));
        Ok(PeerResponse::Found(
            Response::new(peer).with_ref_token(ref_token),
        ))
    }

    pub async fn list(
        context: &SystemContext,
        ListPeers { filters }: &ListPeers,
    ) -> Result<PeerResponse, PeerError> {
        let listed = PeerRepo::new(context).list(filters).await?;
        Ok(PeerResponse::Listed(listed.map(|peer| {
            let ref_token = RefToken::new(Ref::peer(peer.id));
            Response::new(peer).with_ref_token(ref_token)
        })))
    }

    /// Ensure a peer with the given key is known. If one already exists,
    /// return it without emitting an event. Otherwise add it and return
    /// the newly-created record. Used by `bookmark follow` when a remote
    /// URI arrives — the peer gets added transparently if it's new.
    pub async fn ensure(
        context: &SystemContext,
        key: PeerKey,
        address: PeerAddress,
    ) -> Result<Peer, PeerError> {
        let all = SearchFilters {
            limit: Limit(usize::MAX),
            offset: Offset(0),
        };
        let listed = PeerRepo::new(context).list(&all).await?;
        if let Some(existing) = listed.items.iter().find(|p| p.key == key) {
            return Ok(existing.clone());
        }

        let name = default_peer_name(&key);
        let peer = Peer::builder().key(key).address(address).name(name).build();

        context.emit(PeerEvents::PeerAdded(peer.clone())).await?;

        Ok(peer)
    }

    pub async fn remove(
        context: &SystemContext,
        RemovePeer { id }: &RemovePeer,
    ) -> Result<PeerResponse, PeerError> {
        // Verify the peer exists before emitting — keeps the event stream
        // consistent with reality.
        let existing = PeerRepo::new(context)
            .get(*id)
            .await?
            .ok_or(PeerError::NotFound(*id))?;

        context
            .emit(PeerEvents::PeerRemoved(PeerRemoved { id: existing.id }))
            .await?;

        Ok(PeerResponse::Removed(*id))
    }
}

/// Default peer name derived from the first 6 hex digits of the key.
/// Gives human-legible output for `peer list` without requiring users to
/// pick a label for every peer they follow.
fn default_peer_name(key: &PeerKey) -> PeerName {
    let hex = key.to_string();
    let prefix: String = hex.chars().take(6).collect();
    PeerName::new(format!("peer-{prefix}"))
}
