use std::{fs, path::{Path, PathBuf}};

use serde::{Deserialize, Serialize};

use crate::{Endpoint, Fingerprint, TrustError, TrustStatus};

/// TOML-backed storage for known peer fingerprints and insecure allowlist entries.
///
/// The store uses a Trust-On-First-Use (TOFU) model: calling [`PeerStore::accept`]
/// records the peer's fingerprint on first contact. Subsequent connections with a
/// different fingerprint are reported as [`TrustStatus::Untrusted`].
///
/// # Examples
///
/// ```no_run
/// use oneiros_trust::{Endpoint, Fingerprint, PeerStore, TrustStatus};
///
/// # fn main() -> Result<(), oneiros_trust::TrustError> {
/// let dir = std::path::Path::new("/tmp/peers");
/// let mut store = PeerStore::load(dir)?;
///
/// let endpoint = Endpoint::from("192.168.1.50:2100");
/// let fingerprint = Fingerprint::from("sha256:abc123".to_string());
///
/// store.accept(&endpoint, fingerprint.clone())?;
/// assert_eq!(store.status_with_fingerprint(&endpoint, &fingerprint), TrustStatus::Secure);
/// # Ok(())
/// # }
/// ```
pub struct PeerStore {
    peers_dir: PathBuf,
    data: PeerData,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct PeerData {
    #[serde(default)]
    trusted: Vec<TrustedPeer>,
    #[serde(default)]
    insecure: Vec<InsecurePeer>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TrustedPeer {
    endpoint: String,
    fingerprint: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct InsecurePeer {
    endpoint: String,
}

impl PeerStore {
    /// Load the peer store from `peers_dir/known_fingerprints.toml`.
    ///
    /// Creates an empty store when the file does not exist.
    pub fn load(peers_dir: &Path) -> Result<Self, TrustError> {
        let path = peers_dir.join("known_fingerprints.toml");

        let data = if path.exists() {
            let text = fs::read_to_string(&path)
                .map_err(|e| TrustError::StorageFailed(e.into()))?;
            toml::from_str(&text)
                .map_err(|e| TrustError::StorageFailed(e.into()))?
        } else {
            PeerData::default()
        };

        Ok(Self {
            peers_dir: peers_dir.to_owned(),
            data,
        })
    }

    /// Check the trust status for an endpoint without a fingerprint.
    ///
    /// Returns [`TrustStatus::InsecureAllowed`] if the endpoint is in the
    /// insecure allowlist, otherwise [`TrustStatus::Unknown`].
    pub fn status(&self, endpoint: &Endpoint) -> TrustStatus {
        let key = endpoint.to_string();
        if self.data.insecure.iter().any(|p| p.endpoint == key) {
            TrustStatus::InsecureAllowed
        } else {
            TrustStatus::Unknown
        }
    }

    /// Check the trust status for an endpoint with a known fingerprint.
    ///
    /// - Trusted and fingerprint matches → [`TrustStatus::Secure`]
    /// - Trusted but fingerprint differs → [`TrustStatus::Untrusted`]
    /// - Not trusted but in insecure allowlist → [`TrustStatus::InsecureAllowed`]
    /// - Otherwise → [`TrustStatus::Unknown`]
    pub fn status_with_fingerprint(
        &self,
        endpoint: &Endpoint,
        actual: &Fingerprint,
    ) -> TrustStatus {
        let key = endpoint.to_string();

        if let Some(peer) = self.data.trusted.iter().find(|p| p.endpoint == key) {
            if peer.fingerprint == actual.0 {
                TrustStatus::Secure
            } else {
                TrustStatus::Untrusted {
                    expected: Fingerprint::from(peer.fingerprint.clone()),
                    actual: actual.clone(),
                }
            }
        } else if self.data.insecure.iter().any(|p| p.endpoint == key) {
            TrustStatus::InsecureAllowed
        } else {
            TrustStatus::Unknown
        }
    }

    /// Accept a peer by recording its fingerprint, then persist to disk.
    ///
    /// If the endpoint already exists in the trusted list, its fingerprint is updated.
    pub fn accept(
        &mut self,
        endpoint: &Endpoint,
        fingerprint: Fingerprint,
    ) -> Result<(), TrustError> {
        let key = endpoint.to_string();

        if let Some(peer) = self.data.trusted.iter_mut().find(|p| p.endpoint == key) {
            peer.fingerprint = fingerprint.0;
        } else {
            self.data.trusted.push(TrustedPeer {
                endpoint: key,
                fingerprint: fingerprint.0,
            });
        }

        self.save()
    }

    /// Add an endpoint to the insecure allowlist, then persist to disk.
    ///
    /// Idempotent: calling this more than once for the same endpoint has no effect.
    pub fn allow_insecure(&mut self, endpoint: &Endpoint) -> Result<(), TrustError> {
        let key = endpoint.to_string();

        if !self.data.insecure.iter().any(|p| p.endpoint == key) {
            self.data.insecure.push(InsecurePeer { endpoint: key });
        }

        self.save()
    }

    /// Return health snapshots for all currently pinned peers.
    pub fn known_peers(&self) -> Vec<crate::PeerHealth> {
        self.data
            .trusted
            .iter()
            .map(|p| crate::PeerHealth {
                endpoint: Endpoint::from(p.endpoint.as_str()),
                fingerprint: Fingerprint::from(p.fingerprint.clone()),
            })
            .collect()
    }

    fn save(&self) -> Result<(), TrustError> {
        let path = self.peers_dir.join("known_fingerprints.toml");
        let text = toml::to_string_pretty(&self.data)
            .map_err(|e| TrustError::StorageFailed(e.into()))?;
        fs::write(&path, text).map_err(|e| TrustError::StorageFailed(e.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_store_returns_unknown() {
        let dir = tempfile::TempDir::new().unwrap();
        let store = PeerStore::load(dir.path()).unwrap();
        assert_eq!(
            store.status(&Endpoint::from("192.168.1.50:2100")),
            TrustStatus::Unknown
        );
    }

    #[test]
    fn accepted_peer_returns_secure() {
        let dir = tempfile::TempDir::new().unwrap();
        let mut store = PeerStore::load(dir.path()).unwrap();
        let ep = Endpoint::from("192.168.1.50:2100");
        let fp = Fingerprint::from("sha256:abc123".to_string());
        store.accept(&ep, fp.clone()).unwrap();
        assert_eq!(store.status_with_fingerprint(&ep, &fp), TrustStatus::Secure);
    }

    #[test]
    fn fingerprint_mismatch_returns_untrusted() {
        let dir = tempfile::TempDir::new().unwrap();
        let mut store = PeerStore::load(dir.path()).unwrap();
        let ep = Endpoint::from("192.168.1.50:2100");
        store
            .accept(&ep, Fingerprint::from("sha256:original".to_string()))
            .unwrap();
        let actual = Fingerprint::from("sha256:changed".to_string());
        assert!(matches!(
            store.status_with_fingerprint(&ep, &actual),
            TrustStatus::Untrusted { .. }
        ));
    }

    #[test]
    fn insecure_allowed_returns_status() {
        let dir = tempfile::TempDir::new().unwrap();
        let mut store = PeerStore::load(dir.path()).unwrap();
        let ep = Endpoint::from("10.0.0.5:2100");
        store.allow_insecure(&ep).unwrap();
        assert_eq!(store.status(&ep), TrustStatus::InsecureAllowed);
    }

    #[test]
    fn persists_across_loads() {
        let dir = tempfile::TempDir::new().unwrap();
        let ep = Endpoint::from("192.168.1.50:2100");
        let fp = Fingerprint::from("sha256:abc123".to_string());
        {
            let mut store = PeerStore::load(dir.path()).unwrap();
            store.accept(&ep, fp.clone()).unwrap();
        }
        let store = PeerStore::load(dir.path()).unwrap();
        assert_eq!(store.status_with_fingerprint(&ep, &fp), TrustStatus::Secure);
    }
}
