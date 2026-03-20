//! Shared test cases for vocabulary domains (level, texture, sensation, nature, persona, urge).
//!
//! All vocabulary domains share the same CRUD shape: set, show, list, remove.
//! These helpers parameterize by domain command and typed response matchers.

use oneiros_engine::*;
use oneiros_usage::*;

/// Typed response matchers for a vocabulary domain.
///
/// Each field is a plain function pointer so `VocabularyDomain` can be declared
/// as a `const` in the per-domain test modules that call these helpers.
pub struct VocabularyDomain {
    /// The CLI subcommand name (e.g. "level", "texture").
    pub command: &'static str,
    /// Returns `true` when the response is the Set variant for this domain.
    pub is_set: fn(&Responses) -> bool,
    /// Returns `true` when the response is the Details variant for this domain.
    pub is_details: fn(&Responses) -> bool,
    /// Extracts `(name, description, prompt)` from the Details variant, or `None`.
    pub extract_details: fn(&Responses) -> Option<(String, String, String)>,
    /// Returns `true` when the response is the non-empty List variant for this domain.
    pub is_list: fn(&Responses) -> bool,
    /// Extracts the entry count from the List variant, or `None`.
    pub extract_list_count: fn(&Responses) -> Option<usize>,
    /// Returns `true` when the response is the Empty variant for this domain.
    pub is_empty: fn(&Responses) -> bool,
    /// Returns `true` when the response is the Removed variant for this domain.
    pub is_removed: fn(&Responses) -> bool,
}

pub async fn set_creates_a_new_entry<B: Backend>(domain: &VocabularyDomain) -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    let cmd = format!(
        "{} set test-entry --description 'A test entry' --prompt 'Test prompt.'",
        domain.command
    );
    let set_response = backend.exec(&cmd).await?;

    assert!(
        (domain.is_set)(&set_response.data),
        "expected Set response for {}, got {set_response:?}",
        domain.command
    );

    // Verify via show
    let show_cmd = format!("{} show test-entry", domain.command);
    let show_response = backend.exec(&show_cmd).await?;

    let (name, description, _prompt) = (domain.extract_details)(&show_response.data)
        .unwrap_or_else(|| {
            panic!(
                "expected Details response for {}, got {show_response:?}",
                domain.command
            )
        });

    assert_eq!(name, "test-entry");
    assert_eq!(description, "A test entry");

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

    let show_cmd = format!("{} show updatable", domain.command);
    let show_response = backend.exec(&show_cmd).await?;

    let (_name, description, _prompt) = (domain.extract_details)(&show_response.data)
        .unwrap_or_else(|| {
            panic!(
                "expected Details response for {}, got {show_response:?}",
                domain.command
            )
        });

    assert_eq!(description, "Updated");

    Ok(())
}

pub async fn list_returns_empty_when_none_exist<B: Backend>(
    domain: &VocabularyDomain,
) -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    let cmd = format!("{} list", domain.command);
    let response = backend.exec(&cmd).await?;

    assert!(
        (domain.is_empty)(&response.data),
        "expected Empty response for {}, got {response:#?}",
        domain.command
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

    let list_cmd = format!("{} list", domain.command);
    let response = backend.exec(&list_cmd).await?;

    let count = (domain.extract_list_count)(&response.data).unwrap_or_else(|| {
        panic!(
            "expected List response for {}, got {response:#?}",
            domain.command
        )
    });

    assert_eq!(count, 2);

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

    let remove_cmd = format!("{} remove temporary", domain.command);
    let remove_response = backend.exec(&remove_cmd).await?;

    assert!(
        (domain.is_removed)(&remove_response.data),
        "expected Removed response for {}, got {remove_response:?}",
        domain.command
    );

    let list_cmd = format!("{} list", domain.command);
    let list_response = backend.exec(&list_cmd).await?;

    assert!(
        (domain.is_empty)(&list_response.data),
        "expected Empty response after removal for {}, got {list_response:?}",
        domain.command
    );

    Ok(())
}
