use std::path::Path;
use std::process::Command;

use crate::TrustError;

// ---------------------------------------------------------------------------
// Trait
// ---------------------------------------------------------------------------

/// Backend for installing root CA certificates into the system trust store.
///
/// Implement this trait to swap in a test double or a non-default platform
/// strategy without changing the calling code.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use oneiros_trust::{SystemTrustStore, TrustStoreBackend};
///
/// let backend = SystemTrustStore;
/// let cmd = backend.install_command(Path::new("/tmp/root.cert.pem"));
/// assert!(!cmd.is_empty());
/// ```
pub trait TrustStoreBackend: Send + Sync {
    /// Attempt to install the certificate at `cert_path` into the system trust
    /// store.
    ///
    /// Returns `Ok(())` on success. On failure the error message should contain
    /// an actionable manual fallback command.
    fn install(&self, cert_path: &Path) -> Result<(), TrustError>;

    /// Return the platform-specific manual install command for the given cert
    /// path.
    ///
    /// This is useful for diagnostics or doctor output when automatic
    /// installation is not possible or was declined.
    fn install_command(&self, cert_path: &Path) -> String;
}

// ---------------------------------------------------------------------------
// SystemTrustStore — default implementation
// ---------------------------------------------------------------------------

/// Default [`TrustStoreBackend`] that delegates to platform-specific system
/// commands.
///
/// - **macOS**: invokes `security add-trusted-cert` against the login keychain.
/// - **Linux**: copies the cert to `/usr/local/share/ca-certificates/` then
///   runs `update-ca-certificates`.
/// - **Other**: returns an unsupported-platform error with manual instructions.
pub struct SystemTrustStore;

impl TrustStoreBackend for SystemTrustStore {
    fn install(&self, cert_path: &Path) -> Result<(), TrustError> {
        install_root_ca(cert_path)
    }

    fn install_command(&self, cert_path: &Path) -> String {
        platform_install_command(cert_path)
    }
}

// ---------------------------------------------------------------------------
// Private platform helpers
// ---------------------------------------------------------------------------

fn platform_install_command(cert_path: &Path) -> String {
    let path = cert_path.display();
    if cfg!(target_os = "macos") {
        format!(
            "security add-trusted-cert -d -r trustRoot \
             -k ~/Library/Keychains/login.keychain-db {path}"
        )
    } else if cfg!(target_os = "linux") {
        format!(
            "sudo cp {path} /usr/local/share/ca-certificates/oneiros-root.crt \
             && sudo update-ca-certificates"
        )
    } else {
        format!("Manual installation required: add {path} to your system trust store")
    }
}

fn install_root_ca(cert_path: &Path) -> Result<(), TrustError> {
    let manual = platform_install_command(cert_path);
    let path_str = cert_path.to_string_lossy();

    if cfg!(target_os = "macos") {
        install_macos(path_str.as_ref(), &manual)
    } else if cfg!(target_os = "linux") {
        install_linux(path_str.as_ref(), &manual)
    } else {
        Err(TrustError::TrustStoreInstallFailed(format!(
            "unsupported platform — install the root CA manually: {manual}"
        )))
    }
}

#[cfg(target_os = "macos")]
fn install_macos(cert_path: &str, manual: &str) -> Result<(), TrustError> {
    let output = Command::new("security")
        .args([
            "add-trusted-cert",
            "-d",
            "-r",
            "trustRoot",
            "-k",
            &expand_home("~/Library/Keychains/login.keychain-db"),
            cert_path,
        ])
        .output()
        .map_err(|e| {
            TrustError::TrustStoreInstallFailed(format!(
                "could not run `security`: {e} — install manually: {manual}"
            ))
        })?;

    if output.status.success() {
        tracing::info!("root CA installed into macOS login keychain: {cert_path}");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(TrustError::TrustStoreInstallFailed(format!(
            "security command failed: {stderr} — install manually: {manual}"
        )))
    }
}

#[cfg(not(target_os = "macos"))]
fn install_macos(_cert_path: &str, _manual: &str) -> Result<(), TrustError> {
    unreachable!("install_macos called on non-macOS platform")
}

#[cfg(target_os = "linux")]
fn install_linux(cert_path: &str, manual: &str) -> Result<(), TrustError> {
    let dest = "/usr/local/share/ca-certificates/oneiros-root.crt";

    std::fs::copy(cert_path, dest).map_err(|e| {
        TrustError::TrustStoreInstallFailed(format!(
            "could not copy cert to {dest}: {e} — install manually: {manual}"
        ))
    })?;

    let output = Command::new("update-ca-certificates")
        .output()
        .map_err(|e| {
            TrustError::TrustStoreInstallFailed(format!(
                "could not run `update-ca-certificates`: {e} — install manually: {manual}"
            ))
        })?;

    if output.status.success() {
        tracing::info!("root CA installed via update-ca-certificates: {cert_path}");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(TrustError::TrustStoreInstallFailed(format!(
            "update-ca-certificates failed: {stderr} — install manually: {manual}"
        )))
    }
}

#[cfg(not(target_os = "linux"))]
fn install_linux(_cert_path: &str, _manual: &str) -> Result<(), TrustError> {
    unreachable!("install_linux called on non-Linux platform")
}

/// Expands a leading `~/` to the user's home directory.
fn expand_home(path: &str) -> String {
    if let Some(suffix) = path.strip_prefix("~/")
        && let Some(home) = std::env::var_os("HOME")
    {
        return format!("{}/{suffix}", home.to_string_lossy());
    }
    path.to_owned()
}

// ---------------------------------------------------------------------------
// Test double
// ---------------------------------------------------------------------------

#[cfg(test)]
pub struct MockTrustStore {
    pub should_succeed: bool,
    pub installed: std::sync::atomic::AtomicBool,
}

#[cfg(test)]
impl TrustStoreBackend for MockTrustStore {
    fn install(&self, _cert_path: &Path) -> Result<(), TrustError> {
        self.installed
            .store(true, std::sync::atomic::Ordering::Relaxed);
        if self.should_succeed {
            Ok(())
        } else {
            Err(TrustError::TrustStoreInstallFailed("mock failure".to_string()))
        }
    }

    fn install_command(&self, cert_path: &Path) -> String {
        format!("mock-install {}", cert_path.display())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_trust_store_install_command_contains_platform_tool() {
        let backend = SystemTrustStore;
        let cmd = backend.install_command(Path::new("/tmp/root.cert.pem"));
        assert!(!cmd.is_empty());
        #[cfg(target_os = "macos")]
        assert!(cmd.contains("security"));
        #[cfg(target_os = "linux")]
        assert!(cmd.contains("update-ca-certificates"));
    }

    #[test]
    fn system_trust_store_returns_actionable_error_on_failure() {
        let backend = SystemTrustStore;
        let result = backend.install(Path::new("/nonexistent/path/root.cert.pem"));
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("trust")
                || err_msg.contains("install")
                || err_msg.contains("security")
        );
    }

    #[test]
    fn mock_trust_store_records_install() {
        let mock = MockTrustStore {
            should_succeed: true,
            installed: std::sync::atomic::AtomicBool::new(false),
        };
        let path = Path::new("/tmp/test.pem");
        mock.install(path).unwrap();
        assert!(mock.installed.load(std::sync::atomic::Ordering::Relaxed));
    }

    #[test]
    fn mock_trust_store_can_fail() {
        let mock = MockTrustStore {
            should_succeed: false,
            installed: std::sync::atomic::AtomicBool::new(false),
        };
        let path = Path::new("/tmp/test.pem");
        assert!(mock.install(path).is_err());
        assert!(mock.installed.load(std::sync::atomic::Ordering::Relaxed));
    }

    #[test]
    fn mock_trust_store_install_command_uses_path() {
        let mock = MockTrustStore {
            should_succeed: true,
            installed: std::sync::atomic::AtomicBool::new(false),
        };
        let path = Path::new("/tmp/test.pem");
        let cmd = mock.install_command(path);
        assert!(cmd.contains("/tmp/test.pem"));
    }
}
