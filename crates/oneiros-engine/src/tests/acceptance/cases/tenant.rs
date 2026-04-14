use super::*;

pub(crate) async fn list_after_system_init<B: Backend>() -> TestResult {
    let harness = Harness::<B>::setup_system().await?;

    let response = harness.exec_json("tenant list").await?;

    match response {
        Responses::Tenant(TenantResponse::Listed(tenants)) => {
            assert_eq!(
                tenants.len(),
                1,
                "system init should create exactly one tenant"
            );
            assert_eq!(tenants.items[0].data.name.as_str(), "test");
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
