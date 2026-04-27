use crate::*;

pub struct SearchService;

impl SearchService {
    pub async fn search(
        context: &ProjectContext,
        SearchQuery { query, agent }: &SearchQuery,
    ) -> Result<SearchResponse, SearchError> {
        let agent_id = match agent {
            Some(name) => AgentRepo::new(context).get(name).await?.map(|a| a.id()),
            None => None,
        };

        let results = SearchRepo::new(context)
            .search(query, agent_id.as_ref())
            .await?;
        Ok(SearchResponse::Results(SearchResults {
            query: QueryText::new(query),
            results,
        }))
    }
}
