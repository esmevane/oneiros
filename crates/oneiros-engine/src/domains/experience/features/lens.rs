use crate::*;

pub(crate) struct ExperienceLens<'a> {
    scope: &'a Scope<AtBookmark>,
    canons: &'a CanonIndex,
}

impl<'a> ExperienceLens<'a> {
    pub(crate) fn new(scope: &'a Scope<AtBookmark>, canons: &'a CanonIndex) -> Self {
        Self { scope, canons }
    }

    pub(crate) async fn list(
        &self,
        lens: &str,
        filters: &SearchFilters,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let hits = LensService::select(self.scope, self.canons, lens)
            .await?
            .paginate(filters.offset.0, filters.limit.0);
        let mut items = Vec::new();
        let repo = ExperienceRepo::new(self.scope);
        for hit in hits {
            if let Hit::Entity(EntityHit { entity_ref, .. }) = hit
                && let Ref::V0(Resource::Experience(id)) = entity_ref
                && let Some(experience) = repo.fetch(&id).await?
            {
                items.push(experience);
            }
        }
        if items.is_empty() {
            return Ok(ExperienceResponse::NoExperiences);
        }
        let total = items.len();
        Ok(ExperienceResponse::Experiences(
            ExperiencesResponse::builder_v1()
                .items(items)
                .total(total)
                .build()
                .into(),
        ))
    }
}
