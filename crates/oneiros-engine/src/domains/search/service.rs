use std::collections::HashMap;

use crate::*;

pub struct SearchService;

impl SearchService {
    pub async fn search(
        context: &ProjectLog,
        request: &SearchQuery,
    ) -> Result<SearchResponse, SearchError> {
        let query = request.current()?;
        let agent_id = match &query.agent {
            Some(name) => AgentRepo::new(context.scope()?)
                .get(name)
                .await?
                .map(|a| a.id),
            None => None,
        };

        let query = query.with_facets();
        let SearchHits {
            total,
            hits,
            facets,
        } = SearchRepo::new(context.scope()?)
            .search(&query, agent_id.as_ref())
            .await?;

        let hits = hydrate_hits(context, hits).await?;

        let results = SearchResults {
            query: QueryText::new(query.query.clone().unwrap_or_default()),
            total,
            hits,
            facets,
        };
        Ok(SearchResponse::Results(results.into()))
    }
}

/// Walk the ranked refs once, fan out per-kind `get_many` calls, then
/// reassemble [`Hit`]s in the original FTS5 order. Drops refs whose
/// underlying row has been removed since the index was queried — search
/// shouldn't surface ghosts.
pub(crate) async fn hydrate_hits(
    context: &ProjectLog,
    ranked: Vec<RankedHit>,
) -> Result<Vec<Hit>, SearchError> {
    let mut cognition_ids: Vec<CognitionId> = Vec::new();
    let mut memory_ids: Vec<MemoryId> = Vec::new();
    let mut experience_ids: Vec<ExperienceId> = Vec::new();
    let mut agent_ids: Vec<AgentId> = Vec::new();

    for hit in &ranked {
        let Ref::V0(resource) = &hit.resource_ref;
        match resource {
            Resource::Cognition(id) => cognition_ids.push(*id),
            Resource::Memory(id) => memory_ids.push(*id),
            Resource::Experience(id) => experience_ids.push(*id),
            Resource::Agent(id) => agent_ids.push(*id),
            _ => {}
        }
    }

    let cognitions: HashMap<CognitionId, Cognition> = CognitionRepo::new(context.scope()?)
        .get_many(&cognition_ids)
        .await?
        .into_iter()
        .map(|c| (c.id, c))
        .collect();
    let memories: HashMap<MemoryId, Memory> = MemoryRepo::new(context.scope()?)
        .get_many(&memory_ids)
        .await?
        .into_iter()
        .map(|m| (m.id, m))
        .collect();
    let experiences: HashMap<ExperienceId, Experience> = ExperienceRepo::new(context.scope()?)
        .get_many(&experience_ids)
        .await?
        .into_iter()
        .map(|e| (e.id, e))
        .collect();
    let mut agents: HashMap<AgentId, Agent> = AgentRepo::new(context.scope()?)
        .get_many(&agent_ids)
        .await?
        .into_iter()
        .map(|a| (a.id, a))
        .collect();

    let mut hydrated = Vec::with_capacity(ranked.len());
    let mut cognitions = cognitions;
    let mut memories = memories;
    let mut experiences = experiences;
    for hit in ranked {
        let Ref::V0(resource) = hit.resource_ref;
        match resource {
            Resource::Cognition(id) => {
                if let Some(c) = cognitions.remove(&id) {
                    hydrated.push(Hit::Cognition(c));
                }
            }
            Resource::Memory(id) => {
                if let Some(m) = memories.remove(&id) {
                    hydrated.push(Hit::Memory(m));
                }
            }
            Resource::Experience(id) => {
                if let Some(e) = experiences.remove(&id) {
                    hydrated.push(Hit::Experience(e));
                }
            }
            Resource::Agent(id) => {
                if let Some(a) = agents.remove(&id) {
                    hydrated.push(Hit::Agent(a));
                }
            }
            _ => {}
        }
    }
    Ok(hydrated)
}
