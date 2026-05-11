//! Host identity keys.
//!
//! `HostKey` owns the host's persistent ed25519 secret key — the
//! cryptographic identity from which the iroh `EndpointId` and public
//! `PeerKey` are derived. One key per host, shared across brains,
//! lives at `data_dir/host.key`.

use std::path::{Path, PathBuf};

use crate::*;

const HOST_KEY_FILE: &str = "host.key";

#[derive(Debug, thiserror::Error)]
pub(crate) enum HostKeyError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub(crate) struct HostKey {
    platform: Platform,
}

impl HostKey {
    pub(crate) fn new(platform: Platform) -> Self {
        Self { platform }
    }

    /// Path where the host's secret key is persisted.
    pub(crate) fn path(&self) -> PathBuf {
        self.platform.data_dir().join(HOST_KEY_FILE)
    }

    /// Load the persisted host secret key, if one exists. Returns
    /// `None` when the file is missing (first run, not yet generated).
    pub(crate) fn load(&self) -> Result<Option<iroh::SecretKey>, HostKeyError> {
        let path = self.path();
        if !path.exists() {
            return Ok(None);
        }

        let bytes = self.platform.read(&path)?;
        let bytes: [u8; 32] = bytes.try_into().map_err(|byte_error: Vec<u8>| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "host key file has wrong length: expected 32 bytes, got {}",
                    byte_error.len()
                ),
            )
        })?;

        Ok(Some(iroh::SecretKey::from_bytes(&bytes)))
    }

    /// Load the persisted host secret key, or generate and persist a
    /// fresh one if none exists. Idempotent: subsequent calls return
    /// the same key. Safe to call from either `SystemService::init` or
    /// the server's start path — whichever runs first creates the key.
    ///
    /// On Unix, the key file is written with mode `0o600` (owner-only
    /// read/write). On other platforms, file permissions are left at
    /// the OS default.
    pub(crate) fn ensure(&self) -> Result<iroh::SecretKey, HostKeyError> {
        if let Some(existing) = self.load()? {
            return Ok(existing);
        }

        let path = self.path();
        if let Some(parent) = path.parent() {
            self.platform.ensure_dir(parent)?;
        }

        let secret = iroh::SecretKey::generate();
        let bytes = secret.to_bytes();

        write_with_owner_only_perms(&self.platform, &path, &bytes)?;

        Ok(secret)
    }
}

#[cfg(unix)]
fn write_with_owner_only_perms(
    platform: &Platform,
    path: &Path,
    bytes: &[u8],
) -> std::io::Result<()> {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;

    let mut options = std::fs::OpenOptions::new();
    options.create_new(true).write(true).mode(0o600);
    let mut file = platform.open_with(path, &options)?;
    file.write_all(bytes)
}

#[cfg(not(unix))]
fn write_with_owner_only_perms(
    platform: &Platform,
    path: &Path,
    bytes: &[u8],
) -> std::io::Result<()> {
    platform.write(path, bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_generates_when_missing() {
        let dir = tempfile::tempdir().unwrap();
        let keys = HostKey::new(Platform::new(dir.path()));

        assert!(!keys.path().exists());

        let secret = keys.ensure().unwrap();

        assert!(keys.path().exists());
        assert_eq!(
            keys.path(),
            dir.path().join(HOST_KEY_FILE),
            "host key should live at data_dir/host.key"
        );

        let loaded = keys.load().unwrap().unwrap();
        assert_eq!(secret.to_bytes(), loaded.to_bytes());
    }

    #[test]
    fn ensure_is_idempotent() {
        let dir = tempfile::tempdir().unwrap();
        let keys = HostKey::new(Platform::new(dir.path()));

        let first = keys.ensure().unwrap();
        let second = keys.ensure().unwrap();

        assert_eq!(
            first.to_bytes(),
            second.to_bytes(),
            "repeated ensure calls should return the same key"
        );
    }

    #[test]
    fn load_returns_none_when_missing() {
        let dir = tempfile::tempdir().unwrap();
        let keys = HostKey::new(Platform::new(dir.path()));

        let result = keys.load().unwrap();
        assert!(result.is_none());
    }

    #[cfg(unix)]
    #[test]
    fn key_file_has_restricted_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempfile::tempdir().unwrap();
        let keys = HostKey::new(Platform::new(dir.path()));

        keys.ensure().unwrap();

        let metadata = Platform::new(dir.path()).metadata(keys.path()).unwrap();
        let mode = metadata.permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "host key file should be owner-only (0o600)");
    }
}
