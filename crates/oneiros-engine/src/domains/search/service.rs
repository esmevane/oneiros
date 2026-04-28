use crate::*;

pub struct SearchService;

impl SearchService {
    pub async fn search(
        context: &ProjectContext,
        request: &SearchQuery,
    ) -> Result<SearchResponse, SearchError> {
        let SearchQueryV1 { query, agent } = request.current()?;
        let agent_id = match &agent {
            Some(name) => AgentRepo::new(context).get(name).await?.map(|a| a.id),
            None => None,
        };

        let results = SearchRepo::new(context)
            .search(&query, agent_id.as_ref())
            .await?;
        Ok(SearchResponse::Results(
            ResultsResponse::builder_v1()
                .query(QueryText::new(query))
                .results(results)
                .build()
                .into(),
        ))
    }
}
