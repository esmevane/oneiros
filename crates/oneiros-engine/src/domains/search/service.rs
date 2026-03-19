use crate::*;

pub struct SearchService;

impl SearchService {
    pub fn search(
        ctx: &ProjectContext,
        query: &str,
        agent: Option<&str>,
    ) -> Result<SearchResponse, SearchError> {
        let agent_id = agent
            .map(|name| {
                ctx.with_db(|conn| AgentRepo::new(conn).get(name))
                    .map_err(SearchError::Database)
                    .ok()
                    .flatten()
                    .map(|a| a.id.to_string())
            })
            .flatten();

        let results = ctx
            .with_db(|conn| SearchRepo::new(conn).search(query, agent_id.as_deref()))
            .map_err(SearchError::Database)?;
        Ok(SearchResponse::Results(SearchResults {
            query: query.to_string(),
            results,
        }))
    }
}
