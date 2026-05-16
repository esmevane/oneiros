use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn doctor_reports_token_rejected_from_mcp_json() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let mcp_json = serde_json::json!({
        "mcpServers": {
            "oneiros-local": {
                "type": "http",
                "url": app.mcp_url(),
                "headers": {
                    "Authorization": "Bearer garbage-token"
                }
            }
        }
    });

    let path = McpConfigService::mcp_json_path();
    let platform = Platform::default();
    platform.write(&path, serde_json::to_string_pretty(&mcp_json)?)?;

    let rendered = app.command("doctor").await?;
    let response = rendered.response();

    let Responses::Doctor(DoctorResponse::CheckupStatus(CheckupStatusResponse::V1(details))) =
        response
    else {
        platform.remove_file(&path)?;
        panic!("expected Doctor(CheckupStatus(..)), got {response:#?}");
    };

    let has_rejected = details
        .checks
        .iter()
        .any(|c| matches!(c, DoctorCheck::McpTokenRejected(_)));

    platform.remove_file(&path)?;

    assert!(
        has_rejected,
        "expected McpTokenRejected in checks: {:?}",
        details.checks
    );

    Ok(())
}
