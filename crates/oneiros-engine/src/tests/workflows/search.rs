//! Search workflow — finding things across the brain.
//!
//! Data is indexed as it's created: agents, cognitions, memories.
//! Search finds them by content, optionally filtered by agent.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn search_across_domains() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
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
        .search(&SearchQuery::builder().query("architecture").build())
        .await?
    {
        SearchResponse::Results(r) => assert_eq!(r.results.len(), 1),
    }

    // Search finds agent descriptions
    match client
        .search()
        .search(&SearchQuery::builder().query("Governor").build())
        .await?
    {
        SearchResponse::Results(r) => assert_eq!(r.results.len(), 1),
    }

    // Search with agent filter narrows results
    match client
        .search()
        .search(&SearchQuery {
            query: "typed".to_string(),
            agent: Some(AgentName::new("gov.process")),
        })
        .await?
    {
        SearchResponse::Results(r) => assert_eq!(r.results.len(), 1),
    }

    // Search for something that doesn't exist
    match client
        .search()
        .search(&SearchQuery::builder().query("xyznonexistent").build())
        .await?
    {
        SearchResponse::Results(r) => assert_eq!(r.results.len(), 0),
    }

    Ok(())
}
