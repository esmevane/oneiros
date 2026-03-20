use oneiros_engine::*;
use oneiros_usage::*;

pub(crate) async fn reports_initialized_system<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    let response = backend.exec("doctor").await?;

    match response.data {
        Responses::Doctor(reports) => {
            assert!(
                reports.iter().any(|r| matches!(r, DoctorResponse::Initialized)),
                "expected Initialized report in {reports:?}"
            );
            assert!(
                reports.iter().any(|r| matches!(r, DoctorResponse::DatabaseOk(_))),
                "expected DatabaseOk report in {reports:?}"
            );
            assert!(
                reports.iter().any(|r| matches!(r, DoctorResponse::EventLogReady(_))),
                "expected EventLogReady report in {reports:?}"
            );
        }
        other => panic!("expected Doctor(Vec<DoctorResponse>), got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn reports_uninitialized_system<B: Backend>() -> TestResult {
    let backend = B::start().await?;

    let response = backend.exec("doctor").await?;

    match response.data {
        Responses::Doctor(reports) => {
            assert!(
                reports.iter().any(|r| matches!(r, DoctorResponse::NotInitialized)),
                "expected NotInitialized report in {reports:?}"
            );
        }
        other => panic!("expected Doctor(Vec<DoctorResponse>), got {other:#?}"),
    }

    Ok(())
}
