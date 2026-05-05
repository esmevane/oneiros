use super::*;

/// First vertical slice through the bus: `tenant create` dispatches the
/// `TenantCreated` event through the `Mailbox`, the `HostActor` appends
/// it and applies projections, and the service returns the
/// eventually-consistent record via `TenantRepo::fetch`.
///
/// No phantom state — the returned tenant comes from the projection,
/// not from synthesised request data.
pub(crate) async fn create_dispatches_via_bus<B: Backend>() -> TestResult {
    let harness = Harness::<B>::setup_system().await?;

    let response = harness.exec_json("tenant create acme").await?;

    let tenant = match response {
        Responses::Tenant(TenantResponse::Created(TenantCreatedResponse::V1(created))) => {
            created.tenant
        }
        other => panic!("expected Tenant(Created), got {other:#?}"),
    };

    assert_eq!(tenant.name.as_str(), "acme");

    // Verify the eventually-consistent read path also sees the tenant —
    // this proves it landed in the host db's projection, not just in
    // the response synthesised at the call site.
    let listed = harness.exec_json("tenant list").await?;
    match listed {
        Responses::Tenant(TenantResponse::Listed(TenantsResponse::V1(tenants))) => {
            assert!(
                tenants
                    .items
                    .iter()
                    .any(|t| t.name.as_str() == "acme" && t.id == tenant.id),
                "tenant list should include the bus-created tenant: {tenants:#?}"
            );
        }
        other => panic!("expected Tenant(Listed), got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_after_system_init<B: Backend>() -> TestResult {
    let harness = Harness::<B>::setup_system().await?;

    let response = harness.exec_json("tenant list").await?;

    match response {
        Responses::Tenant(TenantResponse::Listed(TenantsResponse::V1(tenants))) => {
            assert_eq!(
                tenants.items.len(),
                1,
                "system init should create exactly one tenant"
            );
            assert_eq!(tenants.items[0].name.as_str(), "test");
        }
        other => panic!("expected Tenant(Listed), got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::setup_system().await?;

    let prompt = harness.exec_prompt("tenant list").await?;

    assert!(!prompt.is_empty(), "tenant list prompt should not be empty");
    assert!(
        prompt.contains("1 of"),
        "tenant list prompt should describe the tenant count, got: {prompt}"
    );

    Ok(())
}
