//! Host identity keys.
//!
//! `HostKey` owns the host's persistent ed25519 secret key — the
//! cryptographic identity from which the iroh `EndpointId` and public
//! `PeerKey` are derived. One key per host, shared across brains,
//! lives at `data_dir/host.key`.

use std::path::{Path, PathBuf};

const HOST_KEY_FILE: &str = "host.key";

#[derive(Debug, thiserror::Error)]
pub enum HostKeyError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub struct HostKey {
    data_dir: PathBuf,
}

impl HostKey {
    pub fn new(data_dir: impl Into<PathBuf>) -> Self {
        Self {
            data_dir: data_dir.into(),
        }
    }

    /// Path where the host's secret key is persisted.
    pub fn path(&self) -> PathBuf {
        self.data_dir.join(HOST_KEY_FILE)
    }

    /// Load the persisted host secret key, if one exists. Returns
    /// `None` when the file is missing (first run, not yet generated).
    pub fn load(&self) -> Result<Option<iroh::SecretKey>, HostKeyError> {
        let path = self.path();
        if !path.exists() {
            return Ok(None);
        }

        let bytes = std::fs::read(&path)?;
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
    pub fn ensure(&self) -> Result<iroh::SecretKey, HostKeyError> {
        if let Some(existing) = self.load()? {
            return Ok(existing);
        }

        let path = self.path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let secret = iroh::SecretKey::generate();
        let bytes = secret.to_bytes();

        write_with_owner_only_perms(&path, &bytes)?;

        Ok(secret)
    }
}

/// Borrowing constructor for transient use.
impl<'a> From<&'a Path> for HostKey {
    fn from(data_dir: &'a Path) -> Self {
        Self::new(data_dir.to_path_buf())
    }
}

#[cfg(unix)]
fn write_with_owner_only_perms(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;

    let mut file = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(path)?;
    file.write_all(bytes)
}

#[cfg(not(unix))]
fn write_with_owner_only_perms(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    std::fs::write(path, bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_generates_when_missing() {
        let dir = tempfile::tempdir().unwrap();
        let keys = HostKey::new(dir.path());

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
        let keys = HostKey::new(dir.path());

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
        let keys = HostKey::new(dir.path());

        let result = keys.load().unwrap();
        assert!(result.is_none());
    }

    #[cfg(unix)]
    #[test]
    fn key_file_has_restricted_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempfile::tempdir().unwrap();
        let keys = HostKey::new(dir.path());

        keys.ensure().unwrap();

        let metadata = std::fs::metadata(keys.path()).unwrap();
        let mode = metadata.permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "host key file should be owner-only (0o600)");
    }
}
