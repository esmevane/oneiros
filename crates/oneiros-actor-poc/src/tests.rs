use oneiros_actor::spawn;
use oneiros_db::Database;
use oneiros_model::*;

use crate::agent::{AgentActor, AgentError};
use crate::database::Db;

// ── Test helpers ───────────────────────────────────────────────────

fn test_db(dir: &std::path::Path) -> Database {
    Database::create_brain_db(&dir.join("test-brain.db")).expect("create brain db")
}

const PROJECTIONS: &[&[oneiros_db::Projection]] = &[crate::projections::AGENT];

async fn test_db_actor(dir: &std::path::Path) -> Db {
    let db = test_db(dir);
    Db::spawn(db, PROJECTIONS)
}

async fn seed_persona(db: &Db) {
    db.set_persona(
        &PersonaName::new("test-persona"),
        &Description::new("A test persona"),
        &Prompt::new("You are a test."),
    )
    .await;
}

type AgentHandle = oneiros_actor::Handle<AgentRequests, Result<AgentResponses, AgentError>>;

async fn test_agent_actor(db: Db) -> AgentHandle {
    spawn(AgentActor::new(db))
}

// ── Agent actor tests ─────────────────────────────────────────────

#[tokio::test]
async fn create_agent_through_actor() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db_actor(dir.path()).await;
    seed_persona(&db).await;

    let agent = test_agent_actor(db).await;

    let response = agent
        .send(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("The governor"),
            prompt: Prompt::new("You govern."),
        }))
        .await
        .unwrap() // Handle send
        .unwrap(); // Domain result

    match response {
        AgentResponses::AgentCreated(a) => {
            assert_eq!(a.name, AgentName::new("governor"));
            assert_eq!(a.persona, PersonaName::new("test-persona"));
        }
        other => panic!("Expected AgentCreated, got {other:?}"),
    }
}

#[tokio::test]
async fn list_agents_through_actor() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db_actor(dir.path()).await;
    seed_persona(&db).await;

    let agent = test_agent_actor(db).await;

    agent
        .send(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("alice"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap()
        .unwrap();

    agent
        .send(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("bob"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap()
        .unwrap();

    let response = agent
        .send(AgentRequests::ListAgents(ListAgentsRequest))
        .await
        .unwrap()
        .unwrap();

    match response {
        AgentResponses::AgentsListed(agents) => {
            assert_eq!(agents.len(), 2);
        }
        other => panic!("Expected AgentsListed, got {other:?}"),
    }
}

#[tokio::test]
async fn get_agent_not_found() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db_actor(dir.path()).await;

    let agent = test_agent_actor(db).await;

    let result = agent
        .send(AgentRequests::GetAgent(GetAgentRequest {
            name: AgentName::new("nonexistent"),
        }))
        .await
        .unwrap(); // Handle send succeeds

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        AgentError::NotFound(NotFound::Agent(_))
    ));
}

#[tokio::test]
async fn create_agent_missing_persona() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db_actor(dir.path()).await;
    // Deliberately NOT seeding persona

    let agent = test_agent_actor(db).await;

    let result = agent
        .send(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("nonexistent"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap();

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        AgentError::NotFound(NotFound::Persona(_))
    ));
}

#[tokio::test]
async fn create_agent_conflict() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db_actor(dir.path()).await;
    seed_persona(&db).await;

    let agent = test_agent_actor(db).await;

    agent
        .send(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap()
        .unwrap();

    let result = agent
        .send(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap();

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AgentError::Conflict(_)));
}

#[tokio::test]
async fn update_agent_through_actor() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db_actor(dir.path()).await;
    seed_persona(&db).await;

    let agent = test_agent_actor(db).await;

    agent
        .send(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("Original"),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap()
        .unwrap();

    let response = agent
        .send(AgentRequests::UpdateAgent(UpdateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("Updated"),
            prompt: Prompt::new("New prompt"),
        }))
        .await
        .unwrap()
        .unwrap();

    match response {
        AgentResponses::AgentUpdated(a) => {
            assert_eq!(a.description, Description::new("Updated"));
        }
        other => panic!("Expected AgentUpdated, got {other:?}"),
    }
}

#[tokio::test]
async fn remove_agent_through_actor() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db_actor(dir.path()).await;
    seed_persona(&db).await;

    let agent = test_agent_actor(db).await;

    agent
        .send(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap()
        .unwrap();

    let response = agent
        .send(AgentRequests::RemoveAgent(RemoveAgentRequest {
            name: AgentName::new("governor"),
        }))
        .await
        .unwrap()
        .unwrap();

    assert!(matches!(response, AgentResponses::AgentRemoved));

    // Verify it's gone
    let result = agent
        .send(AgentRequests::GetAgent(GetAgentRequest {
            name: AgentName::new("governor"),
        }))
        .await
        .unwrap();

    assert!(result.is_err());
}
