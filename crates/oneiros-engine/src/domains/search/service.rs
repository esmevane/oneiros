use crate::*;

pub struct SearchService;

impl SearchService {
    pub fn search(
        ctx: &ProjectContext,
        query: &str,
        agent: Option<&str>,
    ) -> Result<SearchResponse, SearchError> {
        let results = ctx
            .with_db(|conn| SearchRepo::new(conn).search(query, agent))
            .map_err(SearchError::Database)?;
        Ok(SearchResponse::Results(SearchResults {
            query: query.to_string(),
            results,
        }))
    }
}
