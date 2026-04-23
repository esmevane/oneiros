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
        bootstrap.brains.iter().any(|b| b.name == brain_name),
        "init_project should create the 'test' brain"
    );

    assert!(
        bootstrap.tickets.iter().any(|t| t.brain_name == brain_name),
        "init_project should issue a ticket for the default brain"
    );

    Ok(())
}
