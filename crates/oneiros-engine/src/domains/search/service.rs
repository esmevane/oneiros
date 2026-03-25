use crate::*;

pub struct SearchService;

impl SearchService {
    pub fn search(
        context: &ProjectContext,
        query: &str,
        agent: Option<&AgentName>,
    ) -> Result<SearchResponse, SearchError> {
        let agent_id = agent
            .and_then(|name| {
                context
                    .with_db(|conn| AgentRepo::new(conn).get(name))
                    .map_err(SearchError::Database)
                    .ok()
                    .flatten()
                    .map(|a| a.id)
            });

        let results =
            context.with_db(|conn| SearchRepo::new(conn).search(query, agent_id.as_ref()))?;
        Ok(SearchResponse::Results(SearchResults {
            query: query.to_string(),
            results,
        }))
    }
}
