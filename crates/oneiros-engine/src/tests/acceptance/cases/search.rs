use super::*;

pub(crate) async fn search_prompt_contains_results<B: Backend>() -> TestResult {
    let harness = with_searchable::<B>().await?;

    harness
        .exec_json("cognition add thinker.process observation 'The garden is growing well today'")
        .await?;

    let prompt = harness.exec_prompt("search garden").await?;

    assert!(
        !prompt.is_empty(),
        "search prompt should not be empty when results exist"
    );
    assert!(
        prompt.contains("garden"),
        "search prompt should contain the query or matching content"
    );

    Ok(())
}

pub(crate) async fn search_prompt_empty_results<B: Backend>() -> TestResult {
    let harness = with_searchable::<B>().await?;

    let prompt = harness.exec_prompt("search xyznonexistent").await?;

    assert!(
        !prompt.is_empty(),
        "search prompt should not be empty even with no results — tell the agent what happened"
    );

    Ok(())
}

/// Helper: bootstrap with agent and vocabulary for search tests.
async fn with_searchable<B: Backend>() -> Result<Harness<B>, Box<dyn core::error::Error>> {
    let harness = Harness::<B>::init_project().await?;
    harness
        .exec_json("persona set process --description 'Process agents'")
        .await
        .map_err(|e| -> Box<dyn core::error::Error> { e.to_string().into() })?;
    harness
        .exec_json("texture set observation --description 'Observations'")
        .await
        .map_err(|e| -> Box<dyn core::error::Error> { e.to_string().into() })?;
    harness
        .exec_json("level set session --description 'Session context' --prompt 'For the session.'")
        .await
        .map_err(|e| -> Box<dyn core::error::Error> { e.to_string().into() })?;
    harness
        .exec_json("sensation set caused --description 'One thought produced another'")
        .await
        .map_err(|e| -> Box<dyn core::error::Error> { e.to_string().into() })?;
    harness
        .exec_json("agent create thinker process --description 'A thinking agent'")
        .await
        .map_err(|e| -> Box<dyn core::error::Error> { e.to_string().into() })?;
    Ok(harness)
}

/// Helper: extract the results vec from a search response.
fn extract_results(response: Responses) -> Vec<Expression> {
    match response {
        Responses::Search(SearchResponse::Results(search_results)) => search_results.hits,
        other => panic!("expected Search(Results), got {other:#?}"),
    }
}

fn extract_full(response: Responses) -> SearchResults {
    match response {
        Responses::Search(SearchResponse::Results(search_results)) => search_results,
        other => panic!("expected Search(Results), got {other:#?}"),
    }
}

pub(crate) async fn finds_cognition_content<B: Backend>() -> TestResult {
    let harness = with_searchable::<B>().await?;

    harness
        .exec_json("cognition add thinker.process observation 'The architecture is event-sourced'")
        .await?;

    let response = harness.exec_json("search architecture").await?;
    let results = extract_results(response);

    assert_eq!(results.len(), 1, "expected 1 result for cognition content");

    Ok(())
}

pub(crate) async fn finds_memory_content<B: Backend>() -> TestResult {
    let harness = with_searchable::<B>().await?;

    harness
        .exec_json("memory add thinker.process session 'Projections rebuild from events'")
        .await?;

    let response = harness.exec_json("search projections").await?;
    let results = extract_results(response);

    assert_eq!(results.len(), 1, "expected 1 result for memory content");

    Ok(())
}

pub(crate) async fn finds_experience_description<B: Backend>() -> TestResult {
    let harness = with_searchable::<B>().await?;

    harness
        .exec_json("experience create thinker.process caused 'Discovered the replay invariant'")
        .await?;

    let response = harness.exec_json("search replay").await?;
    let results = extract_results(response);

    assert_eq!(
        results.len(),
        1,
        "expected 1 result for experience description"
    );

    Ok(())
}

pub(crate) async fn finds_agent_description<B: Backend>() -> TestResult {
    let harness = with_searchable::<B>().await?;

    // The agent "thinker.process" has description "A thinking agent"
    let response = harness.exec_json("search thinking").await?;
    let results = extract_results(response);

    assert!(
        !results.is_empty(),
        "expected at least 1 result for agent description"
    );

    Ok(())
}

pub(crate) async fn finds_persona_description<B: Backend>() -> TestResult {
    let harness = with_searchable::<B>().await?;

    // The persona "process" has description "Process agents"
    let response = harness.exec_json("search Process").await?;
    let results = extract_results(response);

    assert!(
        !results.is_empty(),
        "expected at least 1 result for persona description"
    );

    Ok(())
}

pub(crate) async fn returns_empty_for_no_match<B: Backend>() -> TestResult {
    let harness = with_searchable::<B>().await?;

    harness
        .exec_json("cognition add thinker.process observation 'Nothing relevant here'")
        .await?;

    let response = harness.exec_json("search xylophone").await?;
    let results = extract_results(response);

    assert!(results.is_empty(), "expected 0 results");

    Ok(())
}

pub(crate) async fn filters_by_agent<B: Backend>() -> TestResult {
    let harness = with_searchable::<B>().await?;

    harness
        .exec_json("agent create other process --description 'Other agent'")
        .await?;

    harness
        .exec_json("cognition add thinker.process observation 'Shared keyword searchable'")
        .await?;
    harness
        .exec_json("cognition add other.process observation 'Also searchable content'")
        .await?;

    let response = harness
        .exec_json("search searchable --agent thinker.process")
        .await?;
    let results = extract_results(response);

    assert_eq!(results.len(), 1, "expected 1 result filtered by agent");

    Ok(())
}

/// Search should find updated agent descriptions (engine doesn't index AgentUpdated).
pub(crate) async fn finds_updated_agent_description<B: Backend>() -> TestResult {
    let harness = with_searchable::<B>().await?;

    // Update the agent's description to something unique and searchable
    harness
        .exec_json("agent update thinker.process process --description 'A uniquely refactored orchestrator'")
        .await?;

    let response = harness.exec_json("search orchestrator").await?;
    let results = extract_results(response);

    assert_eq!(
        results.len(),
        1,
        "expected 1 result for updated agent description"
    );

    Ok(())
}

/// Search response carries faceted aggregations across kind, texture, level,
/// and sensation — the palace map of what's out there.
pub(crate) async fn returns_faceted_results<B: Backend>() -> TestResult {
    let harness = with_searchable::<B>().await?;

    harness
        .exec_json("cognition add thinker.process observation 'Indexable observation one'")
        .await?;
    harness
        .exec_json("cognition add thinker.process observation 'Indexable observation two'")
        .await?;
    harness
        .exec_json("memory add thinker.process session 'Indexable memory session'")
        .await?;
    harness
        .exec_json("experience create thinker.process caused 'Indexable experience caused'")
        .await?;

    let response = harness.exec_json("search Indexable").await?;
    let results = extract_full(response);

    assert_eq!(results.hits.len(), 4, "expected 4 hits across kinds");
    assert_eq!(results.total, 4);

    let kind_group = results
        .facets
        .find(FacetName::Kind)
        .expect("kind facet present");
    assert_eq!(kind_group.buckets.len(), 3, "cognition, memory, experience");

    let texture_group = results
        .facets
        .find(FacetName::Texture)
        .expect("texture facet present");
    assert_eq!(
        texture_group.buckets.len(),
        1,
        "one texture (observation) appears"
    );
    assert_eq!(texture_group.buckets[0].value, "observation");
    assert_eq!(texture_group.buckets[0].count, 2);

    let level_group = results
        .facets
        .find(FacetName::Level)
        .expect("level facet present");
    assert_eq!(level_group.buckets.len(), 1);
    assert_eq!(level_group.buckets[0].value, "session");

    let sensation_group = results
        .facets
        .find(FacetName::Sensation)
        .expect("sensation facet present");
    assert_eq!(sensation_group.buckets.len(), 1);
    assert_eq!(sensation_group.buckets[0].value, "caused");

    Ok(())
}

/// Filtering by `--kind` narrows both hits and every facet group to only
/// rows of that kind.
pub(crate) async fn narrows_by_kind_filter<B: Backend>() -> TestResult {
    let harness = with_searchable::<B>().await?;

    harness
        .exec_json("cognition add thinker.process observation 'Narrowable content here'")
        .await?;
    harness
        .exec_json("memory add thinker.process session 'Narrowable memory content'")
        .await?;

    let response = harness
        .exec_json("search Narrowable --kind cognition")
        .await?;
    let results = extract_full(response);

    assert_eq!(results.hits.len(), 1);
    assert_eq!(results.total, 1);

    let kind_group = results
        .facets
        .find(FacetName::Kind)
        .expect("kind facet present");
    assert_eq!(kind_group.buckets.len(), 1);
    assert_eq!(kind_group.buckets[0].value, "cognition");

    assert!(
        results.facets.find(FacetName::Level).is_none(),
        "level facet should not appear when no memory hits remain"
    );

    Ok(())
}

/// Hits carry typed per-kind metadata — the caller can see texture/level/
/// sensation on each match without a second query.
pub(crate) async fn hits_carry_typed_metadata<B: Backend>() -> TestResult {
    let harness = with_searchable::<B>().await?;

    harness
        .exec_json("cognition add thinker.process observation 'Distinctive cognition text'")
        .await?;

    let response = harness.exec_json("search Distinctive").await?;
    let results = extract_full(response);

    let hit = results
        .hits
        .first()
        .expect("one hit for distinctive content");
    assert_eq!(
        hit.texture.as_ref().map(|t| t.to_string()),
        Some("observation".to_string())
    );
    assert!(hit.agent.is_some(), "agent id populated on hit");
    assert!(hit.created_at.is_some(), "created_at populated on hit");

    Ok(())
}

/// Search should find experience description updates (engine doesn't index ExperienceDescriptionUpdated).
pub(crate) async fn finds_updated_experience_description<B: Backend>() -> TestResult {
    let harness = with_searchable::<B>().await?;

    // Create an experience, then update its description
    harness
        .exec_json("cognition add thinker.process observation 'A thought'")
        .await?;

    let response = harness
        .exec_json("experience create thinker.process caused 'Initial description'")
        .await?;

    let exp_id = match response {
        Responses::Experience(ExperienceResponse::ExperienceCreated(exp)) => {
            exp.data.id.to_string()
        }
        other => panic!("expected ExperienceCreated, got {other:#?}"),
    };

    harness
        .exec_json(&format!(
            "experience update {exp_id} --description 'A completely revised categorization'"
        ))
        .await?;

    let response = harness.exec_json("search categorization").await?;
    let results = extract_results(response);

    assert_eq!(
        results.len(),
        1,
        "expected 1 result for updated experience description"
    );

    Ok(())
}
