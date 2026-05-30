use crate::*;

pub(crate) struct CognitionLens<'a> {
    scope: &'a Scope<AtBookmark>,
    canons: &'a CanonIndex,
}

impl<'a> CognitionLens<'a> {
    pub(crate) fn new(scope: &'a Scope<AtBookmark>, canons: &'a CanonIndex) -> Self {
        Self { scope, canons }
    }

    pub(crate) async fn list(
        &self,
        lens: &str,
        filters: &SearchFilters,
    ) -> Result<CognitionResponse, CognitionError> {
        let hits = LensService::select(self.scope, self.canons, lens)
            .await?
            .paginate(filters.offset.0, filters.limit.0);
        let mut items = Vec::new();
        let repo = CognitionRepo::new(self.scope);
        for hit in hits {
            if let Hit::Entity(EntityHit { entity_ref, .. }) = hit
                && let Ref::V0(Resource::Cognition(id)) = entity_ref
                && let Some(cognition) = repo.fetch(&id).await?
            {
                items.push(cognition);
            }
        }
        if items.is_empty() {
            return Ok(CognitionResponse::NoCognitions);
        }
        let total = items.len();
        Ok(CognitionResponse::Cognitions(
            CognitionsResponse::builder_v1()
                .items(items)
                .total(total)
                .build()
                .into(),
        ))
    }
}
