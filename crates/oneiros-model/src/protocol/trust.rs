use serde::{Deserialize, Serialize};

/// System-level trust events.
///
/// These are stored in the system database, not brain databases.
/// They track the trust configuration lifecycle: mode selection,
/// CA initialization, peer trust decisions, and trust store installation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TrustEvents {
    /// Trust mode was configured (auto/local/acme/off).
    TrustModeConfigured(TrustModeConfigured),
    /// Local CA was initialized or loaded.
    TrustCaInitialized(TrustCaInitialized),
    /// A leaf certificate was issued for a hostname.
    TrustLeafIssued(TrustLeafIssued),
    /// Root CA was installed to the system trust store.
    TrustStoreInstalled,
    /// Root CA installation to system trust store failed.
    TrustStoreInstallFailed(TrustStoreInstallFailed),
    /// A peer was trusted (TOFU accept or config seed).
    TrustPeerAccepted(TrustPeerAccepted),
    /// A peer's fingerprint changed (re-pinned).
    TrustPeerFingerprintChanged(TrustPeerFingerprintChanged),
    /// A peer was added to the insecure allowlist.
    TrustPeerInsecureAllowed(TrustPeerInsecureAllowed),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustModeConfigured {
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustCaInitialized {
    pub root_fingerprint: String,
    pub root_storage_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustLeafIssued {
    pub hostname: String,
    pub not_after: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustStoreInstallFailed {
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustPeerAccepted {
    pub endpoint: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustPeerFingerprintChanged {
    pub endpoint: String,
    pub old_fingerprint: String,
    pub new_fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustPeerInsecureAllowed {
    pub endpoint: String,
    pub reason: Option<String>,
}
