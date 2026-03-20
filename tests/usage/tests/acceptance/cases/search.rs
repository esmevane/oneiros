use oneiros_engine::*;
use oneiros_usage::*;

/// Helper: bootstrap with agent and vocabulary for search tests.
async fn setup_searchable<B: Backend>(backend: &mut B) -> TestResult {
    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;
    backend
        .exec("persona set process --description 'Process agents'")
        .await?;
    backend
        .exec("texture set observation --description 'Observations'")
        .await?;
    backend
        .exec("level set session --description 'Session context' --prompt 'For the session.'")
        .await?;
    backend
        .exec("sensation set caused --description 'One thought produced another'")
        .await?;
    backend
        .exec("agent create thinker process --description 'A thinking agent'")
        .await?;
    Ok(())
}

/// Helper: extract the results vec from a search response.
fn extract_results(response: Response<Responses>) -> Vec<SearchResult> {
    match response.data {
        Responses::Search(SearchResponse::Results(search_results)) => search_results.results,
        other => panic!("expected Search(Results), got {other:#?}"),
    }
}

pub(crate) async fn finds_cognition_content<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_searchable(&mut backend).await?;

    backend
        .exec("cognition add thinker.process observation 'The architecture is event-sourced'")
        .await?;

    let response = backend.exec("search architecture").await?;
    let results = extract_results(response);

    assert_eq!(results.len(), 1, "expected 1 result for cognition content");

    Ok(())
}

pub(crate) async fn finds_memory_content<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_searchable(&mut backend).await?;

    backend
        .exec("memory add thinker.process session 'Projections rebuild from events'")
        .await?;

    let response = backend.exec("search projections").await?;
    let results = extract_results(response);

    assert_eq!(results.len(), 1, "expected 1 result for memory content");

    Ok(())
}

pub(crate) async fn finds_experience_description<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_searchable(&mut backend).await?;

    backend
        .exec("experience create thinker.process caused 'Discovered the replay invariant'")
        .await?;

    let response = backend.exec("search replay").await?;
    let results = extract_results(response);

    assert_eq!(
        results.len(),
        1,
        "expected 1 result for experience description"
    );

    Ok(())
}

pub(crate) async fn finds_agent_description<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_searchable(&mut backend).await?;

    // The agent "thinker.process" has description "A thinking agent"
    let response = backend.exec("search thinking").await?;
    let results = extract_results(response);

    assert!(
        results.len() >= 1,
        "expected at least 1 result for agent description"
    );

    Ok(())
}

pub(crate) async fn finds_persona_description<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_searchable(&mut backend).await?;

    // The persona "process" has description "Process agents"
    let response = backend.exec("search Process").await?;
    let results = extract_results(response);

    assert!(
        results.len() >= 1,
        "expected at least 1 result for persona description"
    );

    Ok(())
}

pub(crate) async fn returns_empty_for_no_match<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_searchable(&mut backend).await?;

    backend
        .exec("cognition add thinker.process observation 'Nothing relevant here'")
        .await?;

    let response = backend.exec("search xylophone").await?;
    let results = extract_results(response);

    assert!(results.is_empty(), "expected 0 results");

    Ok(())
}

pub(crate) async fn filters_by_agent<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_searchable(&mut backend).await?;

    backend
        .exec("agent create other process --description 'Other agent'")
        .await?;

    backend
        .exec("cognition add thinker.process observation 'Shared keyword searchable'")
        .await?;
    backend
        .exec("cognition add other.process observation 'Also searchable content'")
        .await?;

    let response = backend
        .exec("search searchable --agent thinker.process")
        .await?;
    let results = extract_results(response);

    assert_eq!(results.len(), 1, "expected 1 result filtered by agent");

    Ok(())
}

/// Search should find updated agent descriptions (engine doesn't index AgentUpdated).
pub(crate) async fn finds_updated_agent_description<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_searchable(&mut backend).await?;

    // Update the agent's description to something unique and searchable
    backend
        .exec("agent update thinker.process process --description 'A uniquely refactored orchestrator'")
        .await?;

    let response = backend.exec("search orchestrator").await?;
    let results = extract_results(response);

    assert_eq!(
        results.len(),
        1,
        "expected 1 result for updated agent description"
    );

    Ok(())
}

/// Search should find experience description updates (engine doesn't index ExperienceDescriptionUpdated).
pub(crate) async fn finds_updated_experience_description<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_searchable(&mut backend).await?;

    // Create an experience, then update its description
    backend
        .exec("cognition add thinker.process observation 'A thought'")
        .await?;

    let response = backend
        .exec("experience create thinker.process caused 'Initial description'")
        .await?;

    let exp_id = match response.data {
        Responses::Experience(ExperienceResponse::ExperienceCreated(exp)) => exp.id.to_string(),
        other => panic!("expected ExperienceCreated, got {other:#?}"),
    };

    backend
        .exec(&format!(
            "experience update {exp_id} --description 'A completely revised categorization'"
        ))
        .await?;

    let response = backend.exec("search categorization").await?;
    let results = extract_results(response);

    assert_eq!(
        results.len(),
        1,
        "expected 1 result for updated experience description"
    );

    Ok(())
}
