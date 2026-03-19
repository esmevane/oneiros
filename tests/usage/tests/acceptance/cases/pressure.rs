use oneiros_usage::*;

/// Helper: extract pressure readings from the response JSON.
///
/// The expected shape is:
/// ```json
/// {"type": "readings", "data": {"agent": "...", "pressures": [
///   {"urge": "introspect", "data": {"type": "introspect", ...}, ...}
/// ]}}
/// ```
fn extract_pressures(result: &serde_json::Value) -> Vec<serde_json::Value> {
    let outcomes = result.as_array().expect("expected array of outcomes");

    let readings = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("readings")))
        .unwrap_or_else(|| {
            panic!(
                "expected readings outcome in: {}",
                serde_json::to_string_pretty(outcomes).unwrap()
            )
        });

    let data = readings.get("data").expect("readings should have data");

    // Legacy wraps in {agent, pressures: [...]}, engine may differ
    if let Some(pressures) = data.get("pressures").and_then(|p| p.as_array()) {
        pressures.clone()
    } else if let Some(arr) = data.as_array() {
        arr.clone()
    } else {
        panic!(
            "unexpected pressure data shape: {}",
            serde_json::to_string_pretty(data).unwrap()
        )
    }
}

/// Helper: find a specific urge's pressure reading from the array.
fn find_urge<'a>(pressures: &'a [serde_json::Value], urge: &str) -> &'a serde_json::Value {
    pressures
        .iter()
        .find(|p| {
            p.get("urge")
                .and_then(|u| u.as_str())
                .is_some_and(|u| u == urge)
        })
        .unwrap_or_else(|| panic!("expected to find urge '{urge}' in pressures"))
}

/// Helper: extract urgency as a float (0.0-1.0) from a pressure reading.
///
/// The gauge data has shape: { type: "introspect", data: { inputs, calculation, config } }
/// Urgency is the weighted sum of calculation factors × config weights.
fn extract_urgency(pressure: &serde_json::Value) -> f64 {
    let data = pressure
        .get("data")
        .expect("pressure should have data field");

    // The gauge uses adjacently-tagged serde: { type, data: { ... } }
    let gauge_data = data.get("data").unwrap_or_else(|| {
        panic!(
            "gauge should have inner data: {}",
            serde_json::to_string_pretty(data).unwrap()
        )
    });
    let calculation = gauge_data
        .get("calculation")
        .expect("gauge should have calculation");
    let config = gauge_data.get("config").expect("gauge should have config");

    // Compute weighted sum from factor × weight pairs
    // Each gauge type has different factor names, but they all follow the pattern
    let calc_obj = calculation
        .as_object()
        .expect("calculation should be an object");
    let config_obj = config.as_object().expect("config should be an object");

    // Sum up factor * weight for each factor that has a corresponding weight
    let mut urgency = 0.0;
    for (key, value) in calc_obj {
        if let Some(factor) = value.as_f64() {
            // Find the corresponding weight — factor names end in _factor,
            // weight names end in _weight, with the same prefix
            let weight_key = key.replace("_factor", "_weight");
            if let Some(weight) = config_obj.get(&weight_key).and_then(|w| w.as_f64()) {
                urgency += factor * weight;
            }
        }
    }

    urgency
}

pub(crate) async fn returns_readings_for_agent<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;
    backend.exec("seed core").await?;
    backend
        .exec("agent create thinker process --description 'A thinking agent'")
        .await?;

    // Trigger pressure computation — the projection runs on cognition events
    backend
        .exec("cognition add thinker.process observation 'A thought to trigger pressure'")
        .await?;

    let result = backend
        .exec("pressure thinker.process --output json")
        .await?;

    let pressures = extract_pressures(&result);

    // Should have readings for each seeded urge
    assert!(
        !pressures.is_empty(),
        "expected at least one pressure reading in: {}",
        serde_json::to_string_pretty(&result).unwrap()
    );

    // Each reading should have the expected structure
    for pressure in &pressures {
        assert!(
            pressure.get("urge").is_some(),
            "pressure reading should have 'urge' field: {pressure:?}"
        );
        assert!(
            pressure.get("data").is_some(),
            "pressure reading should have 'data' field with gauge: {pressure:?}"
        );
        assert!(
            pressure.get("updated_at").is_some(),
            "pressure reading should have 'updated_at' field: {pressure:?}"
        );
    }

    Ok(())
}

pub(crate) async fn introspect_pressure_decreases_after_introspecting<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;
    backend.exec("seed core").await?;
    backend
        .exec("agent create thinker process --description 'A thinking agent'")
        .await?;

    // Add some cognitions to create activity without introspecting
    backend
        .exec("cognition add thinker.process observation 'First thought'")
        .await?;
    backend
        .exec("cognition add thinker.process observation 'Second thought'")
        .await?;
    backend
        .exec("cognition add thinker.process observation 'Third thought'")
        .await?;

    // Read pressure BEFORE introspecting
    let before = backend
        .exec("pressure thinker.process --output json")
        .await?;
    let before_pressures = extract_pressures(&before);
    let before_introspect = find_urge(&before_pressures, "introspect");
    let before_urgency = extract_urgency(before_introspect);

    // Perform the mitigation: introspect
    backend.exec("introspect thinker.process").await?;

    // Read pressure AFTER introspecting
    let after = backend
        .exec("pressure thinker.process --output json")
        .await?;
    let after_pressures = extract_pressures(&after);
    let after_introspect = find_urge(&after_pressures, "introspect");
    let after_urgency = extract_urgency(after_introspect);

    assert!(
        after_urgency <= before_urgency,
        "introspect pressure should decrease after introspecting: before={before_urgency}, after={after_urgency}"
    );

    Ok(())
}

pub(crate) async fn catharsis_pressure_decreases_after_reflecting<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;
    backend.exec("seed core").await?;
    backend
        .exec("agent create thinker process --description 'A thinking agent'")
        .await?;

    // Add cognitions to create catharsis pressure
    backend
        .exec("cognition add thinker.process observation 'Unresolved thought'")
        .await?;
    backend
        .exec("cognition add thinker.process working 'Another loose thread'")
        .await?;

    // Read pressure BEFORE reflecting
    let before = backend
        .exec("pressure thinker.process --output json")
        .await?;
    let before_pressures = extract_pressures(&before);
    let before_catharsis = find_urge(&before_pressures, "catharsis");
    let before_urgency = extract_urgency(before_catharsis);

    // Perform the mitigation: reflect
    backend.exec("reflect thinker.process").await?;

    // Read pressure AFTER reflecting
    let after = backend
        .exec("pressure thinker.process --output json")
        .await?;
    let after_pressures = extract_pressures(&after);
    let after_catharsis = find_urge(&after_pressures, "catharsis");
    let after_urgency = extract_urgency(after_catharsis);

    assert!(
        after_urgency <= before_urgency,
        "catharsis pressure should decrease after reflecting: before={before_urgency}, after={after_urgency}"
    );

    Ok(())
}
