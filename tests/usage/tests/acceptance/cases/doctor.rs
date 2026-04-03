use oneiros_engine::*;
use oneiros_usage::*;

pub(crate) async fn doctor_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let prompt = harness.exec_prompt("doctor").await?;

    assert!(!prompt.is_empty(), "doctor prompt should not be empty");

    Ok(())
}

pub(crate) async fn reports_initialized_system<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness.exec_json("doctor").await?;

    match response {
        Responses::Doctor(DoctorResponse::CheckupStatus(checks)) => {
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
        }
        other => panic!("expected Doctor(CheckupStatus(..)), got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn reports_uninitialized_system<B: Backend>() -> TestResult {
    let harness = Harness::<B>::started().await?;

    let response = harness.exec_json("doctor").await?;

    match response {
        Responses::Doctor(DoctorResponse::CheckupStatus(checks)) => {
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
