use super::*;

pub(crate) async fn doctor_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let prompt = harness.exec_prompt("doctor").await?;

    assert!(!prompt.is_empty(), "doctor prompt should not be empty");

    Ok(())
}

pub(crate) async fn reports_initialized_host<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness.exec_json("doctor").await?;

    match response {
        Responses::Doctor(DoctorResponse::CheckupStatus(CheckupStatusResponse::V1(details))) => {
            let checks = details.checks;
            assert!(
                checks.iter().any(|r| matches!(r, DoctorCheck::Initialized)),
                "expected Initialized check in {checks:?}"
            );
            assert!(
                checks
                    .iter()
                    .any(|r| matches!(r, DoctorCheck::DatabaseOk(_))),
                "expected DatabaseOk check in {checks:?}"
            );
            assert!(
                checks
                    .iter()
                    .any(|r| matches!(r, DoctorCheck::EventLogReady(_))),
                "expected EventLogReady check in {checks:?}"
            );
            assert!(
                checks.iter().any(|r| matches!(r, DoctorCheck::HostKeyOk)),
                "expected HostKeyOk check in {checks:?}"
            );
            assert!(
                checks
                    .iter()
                    .any(|r| matches!(r, DoctorCheck::ServiceRunning)),
                "expected ServiceRunning check in {checks:?}"
            );
        }
        other => panic!("expected Doctor(CheckupStatus(..)), got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn reports_uninitialized_host<B: Backend>() -> TestResult {
    let harness = Harness::<B>::started().await?;

    let response = harness.exec_json("doctor").await?;

    match response {
        Responses::Doctor(DoctorResponse::CheckupStatus(CheckupStatusResponse::V1(details))) => {
            let checks = details.checks;
            assert!(
                checks
                    .iter()
                    .any(|r| matches!(r, DoctorCheck::NotInitialized)),
                "expected NotInitialized check in {checks:?}"
            );
        }
        other => panic!("expected Doctor(CheckupStatus(..)), got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn reports_mcp_missing_when_no_config<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness.exec_json("doctor").await?;

    match response {
        Responses::Doctor(DoctorResponse::CheckupStatus(CheckupStatusResponse::V1(details))) => {
            let checks = details.checks;
            assert!(
                checks.iter().any(|r| matches!(r, DoctorCheck::McpMissing)),
                "expected McpMissing check in {checks:?}"
            );
        }
        other => panic!("expected Doctor(CheckupStatus(..)), got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn doctor_prompt_includes_mcp_section<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let prompt = harness.exec_prompt("doctor").await?;

    assert!(
        prompt.contains("MCP config"),
        "doctor prompt should mention MCP config, got:\n{prompt}"
    );

    Ok(())
}
