use crate::*;

pub(crate) struct MemoryLens<'a> {
    scope: &'a Scope<AtBookmark>,
    canons: &'a CanonIndex,
}

impl<'a> MemoryLens<'a> {
    pub(crate) fn new(scope: &'a Scope<AtBookmark>, canons: &'a CanonIndex) -> Self {
        Self { scope, canons }
    }

    pub(crate) async fn list(
        &self,
        lens: &str,
        filters: &SearchFilters,
    ) -> Result<MemoryResponse, MemoryError> {
        let hits = LensService::select(self.scope, self.canons, lens)
            .await?
            .paginate(filters.offset.0, filters.limit.0);
        let mut items = Vec::new();
        let repo = MemoryRepo::new(self.scope);
        for hit in hits {
            if let Hit::Entity(EntityHit { entity_ref, .. }) = hit
                && let Ref::V0(Resource::Memory(id)) = entity_ref
                && let Some(memory) = repo.fetch(&id).await?
            {
                items.push(memory);
            }
        }
        if items.is_empty() {
            return Ok(MemoryResponse::NoMemories);
        }
        let total = items.len();
        Ok(MemoryResponse::Memories(
            MemoriesResponse::builder_v1()
                .items(items)
                .total(total)
                .build()
                .into(),
        ))
    }
}
