use crate::*;
use crate::tests::acceptance::harness::*;

/// Helper: init project + persona + level + agent for memory tests.
async fn with_agent_and_level<B: Backend>() -> Result<Harness<B>, Box<dyn core::error::Error>> {
    let harness = Harness::<B>::init_project().await?;
    harness
        .exec_json("persona set process --description 'Process agents'")
        .await?;
    harness
        .exec_json("level set session --description 'Session context' --prompt 'For the session.'")
        .await?;
    harness
        .exec_json("agent create learner process --description 'A learning agent'")
        .await?;
    Ok(harness)
}

pub(crate) async fn add_creates_memory<B: Backend>() -> TestResult {
    let harness = with_agent_and_level::<B>().await?;

    let response = harness
        .exec_json("memory add learner.process session 'A test memory'")
        .await?;

    assert!(
        matches!(response, Responses::Memory(MemoryResponse::MemoryAdded(_))),
        "expected MemoryAdded, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let harness = with_agent_and_level::<B>().await?;

    let response = harness.exec_json("memory list").await?;

    assert!(
        matches!(response, Responses::Memory(MemoryResponse::NoMemories)),
        "expected NoMemories, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let harness = with_agent_and_level::<B>().await?;

    harness
        .exec_json("memory add learner.process session 'First memory'")
        .await?;
    harness
        .exec_json("memory add learner.process session 'Second memory'")
        .await?;

    let response = harness.exec_json("memory list").await?;

    match response {
        Responses::Memory(MemoryResponse::Memories(memories)) => {
            assert_eq!(memories.len(), 2);
        }
        other => panic!("expected Memories, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_filters_by_agent<B: Backend>() -> TestResult {
    let harness = with_agent_and_level::<B>().await?;

    harness
        .exec_json("agent create other process --description 'Other agent'")
        .await?;

    harness
        .exec_json("memory add learner.process session 'Learner memory'")
        .await?;
    harness
        .exec_json("memory add other.process session 'Other memory'")
        .await?;

    let response = harness
        .exec_json("memory list --agent learner.process")
        .await?;

    match response {
        Responses::Memory(MemoryResponse::Memories(memories)) => {
            assert_eq!(memories.len(), 1);
        }
        other => panic!("expected Memories, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn show_by_id<B: Backend>() -> TestResult {
    let harness = with_agent_and_level::<B>().await?;

    let add_response = harness
        .exec_json("memory add learner.process session 'Show me this'")
        .await?;

    let id = match add_response {
        Responses::Memory(MemoryResponse::MemoryAdded(memory)) => memory.data.id,
        other => panic!("expected MemoryAdded, got {other:#?}"),
    };

    let show_response = harness.exec_json(&format!("memory show {id}")).await?;

    match show_response {
        Responses::Memory(MemoryResponse::MemoryDetails(memory)) => {
            assert_eq!(memory.data.content.as_str(), "Show me this");
        }
        other => panic!("expected MemoryDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn show_prompt<B: Backend>() -> TestResult {
    let harness = with_agent_and_level::<B>().await?;

    let response = harness
        .exec_json("memory add learner.process session 'Show me this'")
        .await?;
    let id = match response {
        Responses::Memory(MemoryResponse::MemoryAdded(m)) => m.data.id.to_string(),
        other => panic!("expected MemoryAdded, got {other:#?}"),
    };

    let prompt = harness.exec_prompt(&format!("memory show {id}")).await?;

    assert!(!prompt.is_empty(), "memory show prompt should not be empty");

    Ok(())
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    let harness = with_agent_and_level::<B>().await?;
    harness
        .exec_json("memory add learner.process session 'A memory'")
        .await?;

    let prompt = harness
        .exec_prompt("memory list --agent learner.process")
        .await?;

    assert!(
        !prompt.is_empty(),
        "memory list prompt should not be empty when memories exist"
    );

    Ok(())
}

pub(crate) async fn add_prompt_confirms_creation<B: Backend>() -> TestResult {
    let harness = with_agent_and_level::<B>().await?;

    let prompt = harness
        .exec_prompt("memory add learner.process session 'A prompted memory'")
        .await?;

    assert!(
        !prompt.is_empty(),
        "memory add prompt should not be empty — confirm what was recorded"
    );
    assert!(
        prompt.contains("ref:"),
        "memory add prompt should contain a ref token for the created memory"
    );

    Ok(())
}
