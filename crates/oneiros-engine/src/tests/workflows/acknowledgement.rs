//! Acknowledgement workflow — a write answers with the record it wrote.
//!
//! Writes don't read the projection back to build their response: once
//! the event is persisted, the request succeeds with the record carried by
//! the event. A read-back could fail a write that had already happened,
//! and the client's retry would duplicate it.
//!
//! So the response must be exactly what lands. Each test writes through
//! the CLI, reads the record back the way any client would, and compares
//! the two whole. Where the built and projected records could drift —
//! name normalisation, timestamps, content hashes — this is where it
//! shows.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn cognition_add_responds_with_the_cognition_that_landed()
-> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;
    app.commands(&["agent create thinker process"]).await?;

    let written = match app
        .command(r#"cognition add thinker.process observation "Retries mint duplicates""#)
        .await?
        .into_response()
    {
        Responses::Cognition(CognitionResponse::CognitionAdded(CognitionAddedResponse::V1(
            added,
        ))) => added.cognition,
        other => panic!("expected CognitionAdded, got {other:?}"),
    };

    let landed = match app
        .command(&format!("cognition show {}", written.id))
        .await?
        .into_response()
    {
        Responses::Cognition(CognitionResponse::CognitionDetails(
            CognitionDetailsResponse::V1(details),
        )) => details.cognition,
        other => panic!("expected CognitionDetails, got {other:?}"),
    };

    assert_eq!(landed.content.as_str(), "Retries mint duplicates");
    assert_eq!(landed, written);
    Ok(())
}

#[tokio::test]
async fn memory_add_responds_with_the_memory_that_landed() -> Result<(), Box<dyn core::error::Error>>
{
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;
    app.commands(&["agent create thinker process"]).await?;

    let written = match app
        .command(r#"memory add thinker.process session "A persisted write is a successful write""#)
        .await?
        .into_response()
    {
        Responses::Memory(MemoryResponse::MemoryAdded(MemoryAddedResponse::V1(added))) => {
            added.memory
        }
        other => panic!("expected MemoryAdded, got {other:?}"),
    };

    let landed = match app
        .command(&format!("memory show {}", written.id))
        .await?
        .into_response()
    {
        Responses::Memory(MemoryResponse::MemoryDetails(MemoryDetailsResponse::V1(details))) => {
            details.memory
        }
        other => panic!("expected MemoryDetails, got {other:?}"),
    };

    assert_eq!(
        landed.content.as_str(),
        "A persisted write is a successful write"
    );
    assert_eq!(landed, written);
    Ok(())
}

#[tokio::test]
async fn experience_create_responds_with_the_experience_that_landed()
-> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;
    app.commands(&["agent create thinker process"]).await?;

    let written = match app
        .command(r#"experience create thinker.process echoes "The duplicates, explained""#)
        .await?
        .into_response()
    {
        Responses::Experience(ExperienceResponse::ExperienceCreated(
            ExperienceCreatedResponse::V1(created),
        )) => created.experience,
        other => panic!("expected ExperienceCreated, got {other:?}"),
    };

    let landed = match app
        .command(&format!("experience show {}", written.id))
        .await?
        .into_response()
    {
        Responses::Experience(ExperienceResponse::ExperienceDetails(
            ExperienceDetailsResponse::V1(details),
        )) => details.experience,
        other => panic!("expected ExperienceDetails, got {other:?}"),
    };

    assert_eq!(landed.description.as_str(), "The duplicates, explained");
    assert_eq!(landed, written);
    Ok(())
}

#[tokio::test]
async fn agent_create_responds_with_the_agent_that_landed()
-> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let written = match app
        .command(r#"agent create thinker process --description "A thinking agent""#)
        .await?
        .into_response()
    {
        Responses::Agent(AgentResponse::AgentCreated(AgentCreatedResponse::V1(created))) => {
            created.agent
        }
        other => panic!("expected AgentCreated, got {other:?}"),
    };

    let landed = match app
        .command("agent show thinker.process")
        .await?
        .into_response()
    {
        Responses::Agent(AgentResponse::AgentDetails(AgentDetailsResponse::V1(details))) => {
            details.agent
        }
        other => panic!("expected AgentDetails, got {other:?}"),
    };

    assert_eq!(landed.name.as_str(), "thinker.process");
    assert_eq!(landed, written);
    Ok(())
}

#[tokio::test]
async fn storage_set_responds_with_the_entry_that_landed() -> Result<(), Box<dyn core::error::Error>>
{
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?;
    let files = tempfile::tempdir()?;
    let path = files.path().join("acks.md");
    Platform::new(files.path()).write(&path, "tell, then witness")?;

    let written = match app
        .command(&format!(
            r#"storage set notes/acks.md {} --description "Acknowledgement notes""#,
            path.display()
        ))
        .await?
        .into_response()
    {
        Responses::Storage(StorageResponse::StorageSet(StorageSetResponse::V1(set))) => set.entry,
        other => panic!("expected StorageSet, got {other:?}"),
    };

    let landed = match app
        .command("storage show notes/acks.md")
        .await?
        .into_response()
    {
        Responses::Storage(StorageResponse::StorageDetails(StorageDetailsResponse::V1(
            details,
        ))) => details.entry,
        other => panic!("expected StorageDetails, got {other:?}"),
    };

    assert_eq!(landed.description.as_str(), "Acknowledgement notes");
    assert_eq!(landed, written);
    Ok(())
}
