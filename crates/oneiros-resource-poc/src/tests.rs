use oneiros_db::Database;
use oneiros_model::*;
use oneiros_resource::Fulfill;

use crate::*;

/// Create a brain database in a temp directory for testing.
fn test_db(dir: &std::path::Path) -> Database {
    let db_path = dir.join("test-brain.db");
    Database::create_brain_db(&db_path).expect("create brain db")
}

/// Seed a persona so agent creation can pass FK validation.
fn seed_persona(db: &Database) {
    db.set_persona(
        &PersonaName::new("test-persona"),
        &Description::new("A test persona"),
        &Prompt::new("You are a test."),
    )
    .expect("seed persona");
}

fn test_source() -> Source {
    Source::default()
}

fn project_scope(db: &Database) -> ProjectScope<'_> {
    ProjectScope::new(db, test_source(), &[crate::projections::AGENT])
}

#[tokio::test]
async fn create_agent_through_fulfill() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);

    let scope = project_scope(&db);

    let request = AgentRequests::CreateAgent(CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::new("The governor agent"),
        prompt: Prompt::new("You govern."),
    });

    let response = scope.fulfill(request).await.expect("create agent");

    match response {
        AgentResponses::AgentCreated(agent) => {
            assert_eq!(agent.name, AgentName::new("governor"));
            assert_eq!(agent.persona, PersonaName::new("test-persona"));
        }
        other => panic!("Expected AgentCreated, got {other:?}"),
    }
}

#[tokio::test]
async fn list_agents_through_fulfill() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);

    let scope = project_scope(&db);

    // Create two agents
    scope
        .fulfill(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("alice"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap();

    scope
        .fulfill(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("bob"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap();

    let response = scope
        .fulfill(AgentRequests::ListAgents(ListAgentsRequest))
        .await
        .expect("list agents");

    match response {
        AgentResponses::AgentsListed(agents) => {
            assert_eq!(agents.len(), 2);
            let names: Vec<_> = agents.iter().map(|a| a.name.as_ref()).collect();
            assert!(names.contains(&"alice"));
            assert!(names.contains(&"bob"));
        }
        other => panic!("Expected AgentsListed, got {other:?}"),
    }
}

#[tokio::test]
async fn get_agent_through_fulfill() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);

    let scope = project_scope(&db);

    scope
        .fulfill(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("Gov"),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap();

    let response = scope
        .fulfill(AgentRequests::GetAgent(GetAgentRequest {
            name: AgentName::new("governor"),
        }))
        .await
        .expect("get agent");

    match response {
        AgentResponses::AgentFound(agent) => {
            assert_eq!(agent.name, AgentName::new("governor"));
            assert_eq!(agent.description, Description::new("Gov"));
        }
        other => panic!("Expected AgentFound, got {other:?}"),
    }
}

#[tokio::test]
async fn get_agent_not_found() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());

    let scope = project_scope(&db);

    let result = scope
        .fulfill(AgentRequests::GetAgent(GetAgentRequest {
            name: AgentName::new("nonexistent"),
        }))
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        matches!(err, ProjectScopeError::NotFound(NotFound::Agent(_))),
        "Expected NotFound::Agent, got {err:?}"
    );
}

#[tokio::test]
async fn update_agent_through_fulfill() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);

    let scope = project_scope(&db);

    scope
        .fulfill(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("Original"),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap();

    let response = scope
        .fulfill(AgentRequests::UpdateAgent(UpdateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("Updated"),
            prompt: Prompt::new("New prompt"),
        }))
        .await
        .expect("update agent");

    match response {
        AgentResponses::AgentUpdated(agent) => {
            assert_eq!(agent.description, Description::new("Updated"));
            assert_eq!(agent.prompt, Prompt::new("New prompt"));
        }
        other => panic!("Expected AgentUpdated, got {other:?}"),
    }
}

#[tokio::test]
async fn remove_agent_through_fulfill() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);

    let scope = project_scope(&db);

    scope
        .fulfill(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap();

    let response = scope
        .fulfill(AgentRequests::RemoveAgent(RemoveAgentRequest {
            name: AgentName::new("governor"),
        }))
        .await
        .expect("remove agent");

    assert!(matches!(response, AgentResponses::AgentRemoved));

    // Verify it's gone
    let result = scope
        .fulfill(AgentRequests::GetAgent(GetAgentRequest {
            name: AgentName::new("governor"),
        }))
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn create_agent_conflict() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);

    let scope = project_scope(&db);

    scope
        .fulfill(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap();

    let result = scope
        .fulfill(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        matches!(err, ProjectScopeError::Conflict(_)),
        "Expected Conflict, got {err:?}"
    );
}

#[tokio::test]
async fn create_agent_missing_persona() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    // Deliberately NOT seeding persona

    let scope = project_scope(&db);

    let result = scope
        .fulfill(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("nonexistent-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        matches!(err, ProjectScopeError::NotFound(NotFound::Persona(_))),
        "Expected NotFound::Persona, got {err:?}"
    );
}
