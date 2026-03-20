use oneiros_engine::*;
use oneiros_usage::*;

/// Helper: bootstrap with persona + agent + level so memories have references.
async fn setup_with_agent_and_level<B: Backend>(backend: &mut B) -> TestResult {
    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;
    backend
        .exec("persona set process --description 'Process agents'")
        .await?;
    backend
        .exec("level set session --description 'Session context' --prompt 'For the session.'")
        .await?;
    backend
        .exec("agent create learner process --description 'A learning agent'")
        .await?;
    Ok(())
}

pub(crate) async fn add_creates_memory<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_level(&mut backend).await?;

    let response = backend
        .exec("memory add learner.process session 'A test memory'")
        .await?;

    assert!(
        matches!(response.data, Responses::Memory(MemoryResponse::MemoryAdded(_))),
        "expected MemoryAdded, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_level(&mut backend).await?;

    let response = backend.exec("memory list").await?;

    assert!(
        matches!(response.data, Responses::Memory(MemoryResponse::NoMemories)),
        "expected NoMemories, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_level(&mut backend).await?;

    backend
        .exec("memory add learner.process session 'First memory'")
        .await?;
    backend
        .exec("memory add learner.process session 'Second memory'")
        .await?;

    let response = backend.exec("memory list").await?;

    match response.data {
        Responses::Memory(MemoryResponse::Memories(memories)) => {
            assert_eq!(memories.len(), 2);
        }
        other => panic!("expected Memories, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_filters_by_agent<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_level(&mut backend).await?;

    backend
        .exec("agent create other process --description 'Other agent'")
        .await?;

    backend
        .exec("memory add learner.process session 'Learner memory'")
        .await?;
    backend
        .exec("memory add other.process session 'Other memory'")
        .await?;

    let response = backend
        .exec("memory list --agent learner.process")
        .await?;

    match response.data {
        Responses::Memory(MemoryResponse::Memories(memories)) => {
            assert_eq!(memories.len(), 1);
        }
        other => panic!("expected Memories, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn show_by_id<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_level(&mut backend).await?;

    let add_response = backend
        .exec("memory add learner.process session 'Show me this'")
        .await?;

    let id = match add_response.data {
        Responses::Memory(MemoryResponse::MemoryAdded(memory)) => memory.id,
        other => panic!("expected MemoryAdded, got {other:#?}"),
    };

    let show_cmd = format!("memory show {id}");
    let show_response = backend.exec(&show_cmd).await?;

    match show_response.data {
        Responses::Memory(MemoryResponse::MemoryDetails(memory)) => {
            assert_eq!(memory.content.as_str(), "Show me this");
        }
        other => panic!("expected MemoryDetails, got {other:#?}"),
    }

    Ok(())
}
