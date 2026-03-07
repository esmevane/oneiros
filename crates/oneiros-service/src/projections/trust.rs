use oneiros_db::*;
use oneiros_model::*;
use serde_json::Value;

/// Trust projections for the system database.
///
/// These project trust lifecycle events onto the `trust_state` key/value table
/// and the `trust_peers` table. Two projections own their tables and reset them:
/// `MODE_CONFIGURED_PROJECTION` resets `trust_state`, and
/// `PEER_ACCEPTED_PROJECTION` resets `trust_peers`.
pub const ALL: &[Projection] = &[
    MODE_CONFIGURED_PROJECTION,
    CA_INITIALIZED_PROJECTION,
    LEAF_ISSUED_PROJECTION,
    TRUST_STORE_INSTALLED_PROJECTION,
    TRUST_STORE_INSTALL_FAILED_PROJECTION,
    PEER_ACCEPTED_PROJECTION,
    PEER_FINGERPRINT_CHANGED_PROJECTION,
    PEER_INSECURE_ALLOWED_PROJECTION,
];

const MODE_CONFIGURED_PROJECTION: Projection = Projection {
    name: "trust-mode-configured",
    events: &["trust-mode-configured"],
    apply: apply_mode_configured,
    reset: |db| db.reset_trust_state(),
};

fn apply_mode_configured(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let event: TrustModeConfigured = serde_json::from_value(data.clone())?;

    db.set_trust_state("mode", &event.mode)?;

    Ok(())
}

const CA_INITIALIZED_PROJECTION: Projection = Projection {
    name: "trust-ca-initialized",
    events: &["trust-ca-initialized"],
    apply: apply_ca_initialized,
    reset: |_db| Ok(()),
};

fn apply_ca_initialized(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let event: TrustCaInitialized = serde_json::from_value(data.clone())?;

    db.set_trust_state("ca_fingerprint", &event.root_fingerprint)?;

    if let Some(key) = &event.root_storage_key {
        db.set_trust_state("ca_storage_key", key)?;
    }

    Ok(())
}

const LEAF_ISSUED_PROJECTION: Projection = Projection {
    name: "trust-leaf-issued",
    events: &["trust-leaf-issued"],
    apply: apply_leaf_issued,
    reset: |_db| Ok(()),
};

fn apply_leaf_issued(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let event: TrustLeafIssued = serde_json::from_value(data.clone())?;

    db.set_trust_state("leaf_hostname", &event.hostname)?;
    db.set_trust_state("leaf_not_after", &event.not_after)?;

    Ok(())
}

const TRUST_STORE_INSTALLED_PROJECTION: Projection = Projection {
    name: "trust-store-installed",
    events: &["trust-store-installed"],
    apply: apply_trust_store_installed,
    reset: |_db| Ok(()),
};

fn apply_trust_store_installed(db: &Database, _data: &Value) -> Result<(), DatabaseError> {
    db.set_trust_state("trust_store_installed", "true")?;

    Ok(())
}

const TRUST_STORE_INSTALL_FAILED_PROJECTION: Projection = Projection {
    name: "trust-store-install-failed",
    events: &["trust-store-install-failed"],
    apply: apply_trust_store_install_failed,
    reset: |_db| Ok(()),
};

fn apply_trust_store_install_failed(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let event: TrustStoreInstallFailed = serde_json::from_value(data.clone())?;

    db.set_trust_state("trust_store_installed", "false")?;
    db.set_trust_state("trust_store_install_error", &event.reason)?;

    Ok(())
}

const PEER_ACCEPTED_PROJECTION: Projection = Projection {
    name: "trust-peer-accepted",
    events: &["trust-peer-accepted"],
    apply: apply_peer_accepted,
    reset: |db| db.reset_trust_peers(),
};

fn apply_peer_accepted(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let event: TrustPeerAccepted = serde_json::from_value(data.clone())?;

    db.upsert_trust_peer(&event.endpoint, Some(&event.fingerprint), false, None)?;

    Ok(())
}

const PEER_FINGERPRINT_CHANGED_PROJECTION: Projection = Projection {
    name: "trust-peer-fingerprint-changed",
    events: &["trust-peer-fingerprint-changed"],
    apply: apply_peer_fingerprint_changed,
    reset: |_db| Ok(()),
};

fn apply_peer_fingerprint_changed(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let event: TrustPeerFingerprintChanged = serde_json::from_value(data.clone())?;

    db.upsert_trust_peer(&event.endpoint, Some(&event.new_fingerprint), false, None)?;

    Ok(())
}

const PEER_INSECURE_ALLOWED_PROJECTION: Projection = Projection {
    name: "trust-peer-insecure-allowed",
    events: &["trust-peer-insecure-allowed"],
    apply: apply_peer_insecure_allowed,
    reset: |_db| Ok(()),
};

fn apply_peer_insecure_allowed(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let event: TrustPeerInsecureAllowed = serde_json::from_value(data.clone())?;

    db.upsert_trust_peer(&event.endpoint, None, true, event.reason.as_deref())?;

    Ok(())
}
