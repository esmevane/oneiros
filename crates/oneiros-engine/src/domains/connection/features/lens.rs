use crate::*;

pub(crate) struct ConnectionLens<'a> {
    scope: &'a Scope<AtBookmark>,
    canons: &'a CanonIndex,
}

impl<'a> ConnectionLens<'a> {
    pub(crate) fn new(scope: &'a Scope<AtBookmark>, canons: &'a CanonIndex) -> Self {
        Self { scope, canons }
    }

    pub(crate) async fn list(
        &self,
        lens: &str,
        filters: &SearchFilters,
    ) -> Result<ConnectionResponse, ConnectionError> {
        let hits = LensService::select(self.scope, self.canons, lens)
            .await?
            .paginate(filters.offset.0, filters.limit.0);
        let mut items = Vec::new();
        let repo = ConnectionRepo::new(self.scope);
        for hit in hits {
            if let Hit::Entity(EntityHit { entity_ref, .. }) = hit
                && let Ref::V0(Resource::Connection(id)) = entity_ref
                && let Some(connection) = repo.fetch(&id).await?
            {
                items.push(connection);
            }
        }
        if items.is_empty() {
            return Ok(ConnectionResponse::NoConnections);
        }
        let total = items.len();
        Ok(ConnectionResponse::Connections(
            ConnectionsResponse::builder_v1()
                .items(items)
                .total(total)
                .build()
                .into(),
        ))
    }
}
