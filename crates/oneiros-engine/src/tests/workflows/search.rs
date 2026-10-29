//! Search workflow — finding things across the project.
//!
//! Data is indexed as it's created: agents, cognitions, memories.
//! Search finds them by content, optionally filtered by agent.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn search_across_domains() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();

    // Populate data across domains
    app.command(r#"agent create gov process --description "Governor agent""#)
        .await?;
    app.command(r#"cognition add gov.process observation "The architecture is clean""#)
        .await?;
    app.command(r#"cognition add gov.process working "Working on typed events""#)
        .await?;
    app.command(r#"memory add gov.process project "Event sourcing works well here""#)
        .await?;

    // Search finds cognitions by content
    match client
        .search()
        .search(
            &SearchQuery::builder_v1()
                .query("architecture")
                .build()
                .into(),
        )
        .await?
    {
        SearchResponse::Results(ResultsResponse::V1(r)) => assert_eq!(r.hits.len(), 1),
    }

    // Search finds agent descriptions
    match client
        .search()
        .search(&SearchQuery::builder_v1().query("Governor").build().into())
        .await?
    {
        SearchResponse::Results(ResultsResponse::V1(r)) => assert_eq!(r.hits.len(), 1),
    }

    // Search with agent filter narrows results
    match client
        .search()
        .search(
            &SearchQuery::builder_v1()
                .query("typed")
                .agent(AgentName::new("gov.process"))
                .build()
                .into(),
        )
        .await?
    {
        SearchResponse::Results(ResultsResponse::V1(r)) => assert_eq!(r.hits.len(), 1),
    }

    // Search for something that doesn't exist
    match client
        .search()
        .search(
            &SearchQuery::builder_v1()
                .query("xyznonexistent")
                .build()
                .into(),
        )
        .await?
    {
        SearchResponse::Results(ResultsResponse::V1(r)) => assert_eq!(r.hits.len(), 0),
    }

    Ok(())
}

/// Search text is user prose, not FTS5 query syntax. Characters FTS5
/// treats as operators (`#`, `:`, `"`, `*`, `-`, parens, AND/OR/NOT)
/// must be searched for literally rather than parsed.
mod query_syntax {
    use super::*;

    async fn seeded() -> Result<TestApp, Box<dyn core::error::Error>> {
        let app = TestApp::new()
            .await?
            .init_host()
            .await?
            .init_project()
            .await?
            .seed_core()
            .await?;

        app.command(r#"agent create gov process --description "Governor agent""#)
            .await?;
        app.command(r#"cognition add gov.process observation "Picked up issue #317 today""#)
            .await?;
        app.command(
            r#"cognition add gov.process connection "Builds on ref:AAQQAaGCDRAT-search_defect""#,
        )
        .await?;
        app.command(r#"cognition add gov.process observation "Search is plumbing""#)
            .await?;
        app.command(
            r#"cognition add gov.process learning "Search relevance: search ranking search quality""#,
        )
        .await?;

        Ok(app)
    }

    async fn search(
        app: &TestApp,
        text: &str,
    ) -> Result<SearchResults, Box<dyn core::error::Error>> {
        let response = app
            .client()
            .search()
            .search(
                &SearchQuery::builder_v1()
                    .query(text)
                    .kind(SearchKind::Cognition)
                    .build()
                    .into(),
            )
            .await?;
        let SearchResponse::Results(ResultsResponse::V1(results)) = response;
        Ok(results)
    }

    fn contents(results: &SearchResults) -> Vec<String> {
        results
            .hits
            .iter()
            .map(|hit| hit.content().to_string())
            .collect()
    }

    #[tokio::test]
    async fn hash_is_searched_literally() -> Result<(), Box<dyn core::error::Error>> {
        let app = seeded().await?;

        let results = search(&app, "#317").await?;

        assert_eq!(contents(&results), vec!["Picked up issue #317 today"]);
        Ok(())
    }

    #[tokio::test]
    async fn ref_prefix_is_not_a_column_filter() -> Result<(), Box<dyn core::error::Error>> {
        let app = seeded().await?;

        let results = search(&app, "ref:AAQQAaGCDRAT-search_defect").await?;

        assert_eq!(
            contents(&results),
            vec!["Builds on ref:AAQQAaGCDRAT-search_defect"]
        );
        Ok(())
    }

    #[tokio::test]
    async fn unbalanced_quote_does_not_fail() -> Result<(), Box<dyn core::error::Error>> {
        let app = seeded().await?;

        let results = search(&app, r#""plumbing"#).await?;

        assert_eq!(contents(&results), vec!["Search is plumbing"]);
        Ok(())
    }

    #[tokio::test]
    async fn operator_soup_does_not_fail() -> Result<(), Box<dyn core::error::Error>> {
        let app = seeded().await?;

        for text in [
            "*",
            "-",
            "(",
            "AND",
            "NOT search",
            "a OR",
            "NEAR(",
            "^",
            "\"\"",
            "   ",
        ] {
            search(&app, text)
                .await
                .unwrap_or_else(|e| panic!("query {text:?} failed: {e}"));
        }
        Ok(())
    }

    #[tokio::test]
    async fn multi_word_query_ranks_by_relevance() -> Result<(), Box<dyn core::error::Error>> {
        let app = seeded().await?;

        let results = search(&app, "search relevance").await?;

        // Every term must match (implicit AND), so "Search is plumbing" is
        // excluded; the one dense with both terms ranks alone at the top.
        assert_eq!(
            contents(&results),
            vec!["Search relevance: search ranking search quality"]
        );

        let results = search(&app, "search").await?;
        assert_eq!(
            contents(&results).first().map(String::as_str),
            Some("Search relevance: search ranking search quality"),
            "the cognition repeating 'search' should outrank the others"
        );
        // "search_defect" tokenizes to "search" + "defect", so the ref
        // cognition matches too.
        assert_eq!(results.hits.len(), 3);
        Ok(())
    }
}
