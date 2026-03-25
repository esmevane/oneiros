use crate::*;

pub struct SearchService;

impl SearchService {
    pub fn search(
        context: &ProjectContext,
        query: &str,
        agent: Option<&AgentName>,
    ) -> Result<SearchResponse, SearchError> {
        let db = context.db()?;

        let agent_id =
            agent.and_then(|name| AgentRepo::new(&db).get(name).ok().flatten().map(|a| a.id));

        let results = SearchRepo::new(&db).search(query, agent_id.as_ref())?;
        Ok(SearchResponse::Results(SearchResults {
            query: query.to_string(),
            results,
        }))
    }
}
