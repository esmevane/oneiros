use oneiros_engine::*;
use oneiros_usage::*;

/// Helper: extract pressure readings from a `Response<Responses>` value.
fn extract_pressures(response: Response<Responses>) -> Vec<Pressure> {
    match response.data {
        Responses::Pressure(PressureResponse::Readings(result)) => result.pressures,
        other => panic!("expected Pressure(Readings), got {other:#?}"),
    }
}

/// Helper: find a specific urge's pressure reading from the vec.
fn find_urge<'a>(pressures: &'a [Pressure], urge: &str) -> &'a Pressure {
    pressures
        .iter()
        .find(|p| p.urge.as_str() == urge)
        .unwrap_or_else(|| panic!("expected to find urge '{urge}' in pressures"))
}

pub(crate) async fn returns_readings_for_agent<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;
    backend.exec_json("seed core").await?;
    backend
        .exec_json("agent create thinker process --description 'A thinking agent'")
        .await?;

    // Trigger pressure computation — the projection runs on cognition events
    backend
        .exec_json("cognition add thinker.process observation 'A thought to trigger pressure'")
        .await?;

    let response = backend.exec_json("pressure thinker.process").await?;
    let pressures = extract_pressures(response);

    // Should have readings for each seeded urge
    assert!(
        !pressures.is_empty(),
        "expected at least one pressure reading"
    );

    Ok(())
}

pub(crate) async fn introspect_pressure_decreases_after_introspecting<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;
    backend.exec_json("seed core").await?;
    backend
        .exec_json("agent create thinker process --description 'A thinking agent'")
        .await?;

    // Add some cognitions to create activity without introspecting
    backend
        .exec_json("cognition add thinker.process observation 'First thought'")
        .await?;
    backend
        .exec_json("cognition add thinker.process observation 'Second thought'")
        .await?;
    backend
        .exec_json("cognition add thinker.process observation 'Third thought'")
        .await?;

    // Read pressure BEFORE introspecting
    let before_response = backend.exec_json("pressure thinker.process").await?;
    let before_pressures = extract_pressures(before_response);
    let before_introspect = find_urge(&before_pressures, "introspect");
    let before_urgency = before_introspect.urgency();

    // Perform the mitigation: introspect
    backend.exec_json("introspect thinker.process").await?;

    // Read pressure AFTER introspecting
    let after_response = backend.exec_json("pressure thinker.process").await?;
    let after_pressures = extract_pressures(after_response);
    let after_introspect = find_urge(&after_pressures, "introspect");
    let after_urgency = after_introspect.urgency();

    assert!(
        after_urgency <= before_urgency,
        "introspect pressure should decrease after introspecting: before={before_urgency}, after={after_urgency}"
    );

    Ok(())
}

pub(crate) async fn catharsis_pressure_decreases_after_reflecting<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;
    backend.exec_json("seed core").await?;
    backend
        .exec_json("agent create thinker process --description 'A thinking agent'")
        .await?;

    // Add cognitions to create catharsis pressure
    backend
        .exec_json("cognition add thinker.process observation 'Unresolved thought'")
        .await?;
    backend
        .exec_json("cognition add thinker.process working 'Another loose thread'")
        .await?;

    // Read pressure BEFORE reflecting
    let before_response = backend.exec_json("pressure thinker.process").await?;
    let before_pressures = extract_pressures(before_response);
    let before_catharsis = find_urge(&before_pressures, "catharsis");
    let before_urgency = before_catharsis.urgency();

    // Perform the mitigation: reflect
    backend.exec_json("reflect thinker.process").await?;

    // Read pressure AFTER reflecting
    let after_response = backend.exec_json("pressure thinker.process").await?;
    let after_pressures = extract_pressures(after_response);
    let after_catharsis = find_urge(&after_pressures, "catharsis");
    let after_urgency = after_catharsis.urgency();

    assert!(
        after_urgency <= before_urgency,
        "catharsis pressure should decrease after reflecting: before={before_urgency}, after={after_urgency}"
    );

    Ok(())
}
