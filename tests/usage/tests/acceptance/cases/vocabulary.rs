//! Shared test cases for vocabulary domains (level, texture, sensation, nature, persona, urge).
//!
//! All vocabulary domains share the same CRUD shape: set, show, list, remove.
//! These helpers parameterize by domain name and the expected JSON type prefixes.

use oneiros_usage::*;

pub struct VocabularyDomain {
    /// The CLI subcommand name (e.g. "level", "texture").
    pub command: &'static str,
    /// The JSON type prefix for set outcomes (e.g. "level-set", "texture-set").
    pub set_type: &'static str,
    /// The JSON type for show outcomes (e.g. "level-details", "texture-details").
    pub details_type: &'static str,
    /// The JSON type for list outcomes (e.g. "levels", "textures").
    pub list_type: &'static str,
    /// The JSON type for empty list outcomes (e.g. "no-levels", "no-textures").
    pub empty_type: &'static str,
    /// The JSON type for remove outcomes (e.g. "level-removed", "texture-removed").
    pub removed_type: &'static str,
}

pub async fn set_creates_a_new_entry<B: Backend>(domain: &VocabularyDomain) -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    let cmd = format!(
        "{} set test-entry --description 'A test entry' --prompt 'Test prompt.' --output json",
        domain.command
    );
    let result = backend.exec(&cmd).await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!(domain.set_type))),
        "expected {} outcome in {outcomes:?}",
        domain.set_type
    );

    // Verify via show
    let show_cmd = format!("{} show test-entry --output json", domain.command);
    let show_result = backend.exec(&show_cmd).await?;
    let show_outcomes = show_result.as_array().expect("expected array of outcomes");

    let entry = show_outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!(domain.details_type)))
        .unwrap_or_else(|| {
            panic!(
                "expected {} outcome in {show_outcomes:?}",
                domain.details_type
            )
        });

    let data = entry.get("data").expect("expected data field");
    assert_eq!(
        data.get("name").and_then(|n| n.as_str()),
        Some("test-entry")
    );
    assert_eq!(
        data.get("description").and_then(|d| d.as_str()),
        Some("A test entry")
    );

    Ok(())
}

pub async fn set_updates_existing_entry<B: Backend>(domain: &VocabularyDomain) -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    let cmd = format!(
        "{} set updatable --description 'Original' --prompt 'Original.'",
        domain.command
    );
    backend.exec(&cmd).await?;

    let cmd = format!(
        "{} set updatable --description 'Updated' --prompt 'Updated.'",
        domain.command
    );
    backend.exec(&cmd).await?;

    let show_cmd = format!("{} show updatable --output json", domain.command);
    let show_result = backend.exec(&show_cmd).await?;
    let show_outcomes = show_result.as_array().expect("expected array of outcomes");

    let entry = show_outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!(domain.details_type)))
        .unwrap_or_else(|| panic!("expected {} outcome", domain.details_type));

    let data = entry.get("data").expect("expected data field");
    assert_eq!(
        data.get("description").and_then(|d| d.as_str()),
        Some("Updated")
    );

    Ok(())
}

pub async fn list_returns_empty_when_none_exist<B: Backend>(
    domain: &VocabularyDomain,
) -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    let cmd = format!("{} list --output json", domain.command);
    let result = backend.exec(&cmd).await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!(domain.empty_type))),
        "expected {} outcome in {outcomes:?}",
        domain.empty_type
    );

    Ok(())
}

pub async fn list_returns_created_entries<B: Backend>(domain: &VocabularyDomain) -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    let cmd1 = format!(
        "{} set first --description 'First entry' --prompt 'First.'",
        domain.command
    );
    backend.exec(&cmd1).await?;

    let cmd2 = format!(
        "{} set second --description 'Second entry' --prompt 'Second.'",
        domain.command
    );
    backend.exec(&cmd2).await?;

    let list_cmd = format!("{} list --output json", domain.command);
    let result = backend.exec(&list_cmd).await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    let list_outcome = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!(domain.list_type)))
        .unwrap_or_else(|| panic!("expected {} outcome in {outcomes:?}", domain.list_type));

    let entries = list_outcome
        .get("data")
        .and_then(|d| d.as_array())
        .expect("expected data array");

    assert_eq!(entries.len(), 2);

    Ok(())
}

pub async fn remove_makes_it_unlisted<B: Backend>(domain: &VocabularyDomain) -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    let set_cmd = format!(
        "{} set temporary --description 'Will be removed' --prompt 'Temporary.'",
        domain.command
    );
    backend.exec(&set_cmd).await?;

    let remove_cmd = format!("{} remove temporary --output json", domain.command);
    let remove_result = backend.exec(&remove_cmd).await?;
    let remove_outcomes = remove_result
        .as_array()
        .expect("expected array of outcomes");

    assert!(
        remove_outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!(domain.removed_type))),
        "expected {} outcome in {remove_outcomes:?}",
        domain.removed_type
    );

    let list_cmd = format!("{} list --output json", domain.command);
    let list_result = backend.exec(&list_cmd).await?;
    let list_outcomes = list_result.as_array().expect("expected array of outcomes");

    assert!(
        list_outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!(domain.empty_type))),
        "expected {} after removal in {list_outcomes:?}",
        domain.empty_type
    );

    Ok(())
}
