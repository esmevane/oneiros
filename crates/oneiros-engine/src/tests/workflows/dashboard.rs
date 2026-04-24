//! Dashboard workflow — the host-shaped bootstrap endpoint.
//!
//! The dashboard's `/dashboard/config` endpoint hands the web UI
//! everything it needs to render the host view: host identity,
//! tenants, brains on disk, and tickets the host knows tokens for.
//! A single unauthenticated call, joined client-side.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn dashboard_config_returns_host_shape() -> Result<(), Box<dyn core::error::Error>> {
    // Arrange: a system with a project (which creates a brain + a ticket
    // for the default actor).
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    // Act: hit /dashboard/config without auth — it's a SystemContext route.
    let http = reqwest::Client::new();
    let url = format!("{}/dashboard/config", app.base_url());
    let bootstrap: DashboardBootstrap = http
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    // Assert: every section of the host shape is populated from the
    // init-system + init-project sequence.
    assert!(
        !bootstrap.version.is_empty(),
        "version should be the engine crate version"
    );

    let brain_name = BrainName::new("test");
    assert_eq!(
        bootstrap.current_brain, brain_name,
        "current_brain should match the server's configured brain"
    );

    assert!(
        !bootstrap.tenants.is_empty(),
        "init_system should create the default tenant"
    );

    assert!(
        !bootstrap.actors.is_empty(),
        "init_system should create the default actor"
    );

    assert!(
        bootstrap.brains.iter().any(|b| b.name == brain_name),
        "init_project should create the 'test' brain"
    );

    assert!(
        bootstrap.tickets.iter().any(|t| t.brain_name == brain_name),
        "init_project should issue a ticket for the default brain"
    );

    // Peers is always a valid vec, even if empty on a fresh host.
    let _ = bootstrap.peers;

    Ok(())
}

/// The HTML page served at `/` is host-shaped — it contains the host
/// landing page and the brain page, with the host as the default.
/// This is a coarse smoke test, not a UX test: it catches accidental
/// deletion or rewrites that would break the host-first layout.
#[tokio::test]
async fn dashboard_html_has_host_and_brain_pages() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new().await?.init_system().await?;

    let http = reqwest::Client::new();
    let body = http
        .get(app.base_url())
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    assert!(
        body.contains(r#"id="page-host""#),
        "host landing page should be present"
    );
    assert!(
        body.contains(r#"id="page-brain""#),
        "brain page should be present"
    );
    assert!(
        body.contains(r#"id="sb-brain-section""#),
        "sidebar should have a conditionally-visible BRAIN section"
    );
    assert!(
        body.contains("go('host')"),
        "brand click should route to the host page"
    );
    assert!(
        body.contains(r#"id="page-tenants""#),
        "tenants list page should be present"
    );
    assert!(
        body.contains(r#"id="page-tenant""#),
        "tenant detail page should be present"
    );
    assert!(
        body.contains(r#"id="page-actor""#),
        "actor detail page should be present"
    );

    Ok(())
}
