use crate::*;

/// Resolves a lens expression into agent entities. Works for both HTTP
/// (via `state.canons()`) and MCP (via `context.canons()`) callers.
pub(crate) struct AgentLens<'a> {
    scope: &'a Scope<AtBookmark>,
    canons: &'a CanonIndex,
}

impl<'a> AgentLens<'a> {
    pub(crate) fn new(scope: &'a Scope<AtBookmark>, canons: &'a CanonIndex) -> Self {
        Self { scope, canons }
    }

    pub(crate) async fn list(
        &self,
        lens: &str,
        filters: &SearchFilters,
    ) -> Result<AgentResponse, AgentError> {
        let hits = LensService::select(self.scope, self.canons, lens)
            .await?
            .paginate(filters.offset.0, filters.limit.0);
        let mut items = Vec::new();
        let repo = AgentRepo::new(self.scope);
        for hit in hits {
            if let Hit::Entity(EntityHit { entity_ref, .. }) = hit
                && let Ref::V0(Resource::Agent(id)) = entity_ref
                && let Some(agent) = repo.fetch_by_id(id).await?
            {
                items.push(agent);
            }
        }
        if items.is_empty() {
            return Ok(AgentResponse::NoAgents);
        }
        let total = items.len();
        Ok(AgentResponse::Agents(
            AgentsResponse::builder_v1()
                .items(items)
                .total(total)
                .build()
                .into(),
        ))
    }
}
