use std::process::Command;

use crate::TrustError;

/// Returns the platform-specific manual install command for the given cert path.
///
/// This is useful for diagnostics or doctor output when automatic installation
/// is not possible or was declined.
///
/// # Examples
///
/// ```
/// let cmd = oneiros_trust::install_command("/tmp/root.cert.pem");
/// assert!(!cmd.is_empty());
/// ```
pub fn install_command(cert_path: &str) -> String {
    if cfg!(target_os = "macos") {
        format!(
            "security add-trusted-cert -d -r trustRoot \
             -k ~/Library/Keychains/login.keychain-db {cert_path}"
        )
    } else if cfg!(target_os = "linux") {
        format!(
            "sudo cp {cert_path} /usr/local/share/ca-certificates/oneiros-root.crt \
             && sudo update-ca-certificates"
        )
    } else {
        format!(
            "Manual installation required: add {cert_path} to your system trust store"
        )
    }
}

/// Attempts to install the root CA certificate into the system trust store.
///
/// Returns `Ok(true)` on success. On failure the error message contains the
/// manual command needed to complete the installation.
///
/// Platform behaviour:
/// - **macOS**: invokes `security add-trusted-cert` against the login keychain.
/// - **Linux**: copies the cert to `/usr/local/share/ca-certificates/` then runs
///   `update-ca-certificates`.
/// - **Other**: returns an unsupported-platform error with manual instructions.
///
/// # Errors
///
/// Returns [`TrustError::TrustStoreInstallFailed`] if the platform is
/// unsupported or the underlying system command fails. The error message always
/// includes a manual fallback command.
///
/// # Examples
///
/// ```no_run
/// use oneiros_trust::install_root_ca;
///
/// match install_root_ca("/path/to/root.cert.pem") {
///     Ok(true) => println!("installed"),
///     Err(e)   => eprintln!("failed: {e}"),
///     _        => unreachable!(),
/// }
/// ```
pub fn install_root_ca(cert_path: &str) -> Result<bool, TrustError> {
    let manual = install_command(cert_path);

    if cfg!(target_os = "macos") {
        install_macos(cert_path, &manual)
    } else if cfg!(target_os = "linux") {
        install_linux(cert_path, &manual)
    } else {
        Err(TrustError::TrustStoreInstallFailed(format!(
            "unsupported platform — install the root CA manually: {manual}"
        )))
    }
}

#[cfg(target_os = "macos")]
fn install_macos(cert_path: &str, manual: &str) -> Result<bool, TrustError> {
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
        Ok(true)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(TrustError::TrustStoreInstallFailed(format!(
            "security command failed: {stderr} — install manually: {manual}"
        )))
    }
}

#[cfg(not(target_os = "macos"))]
fn install_macos(_cert_path: &str, _manual: &str) -> Result<bool, TrustError> {
    unreachable!("install_macos called on non-macOS platform")
}

#[cfg(target_os = "linux")]
fn install_linux(cert_path: &str, manual: &str) -> Result<bool, TrustError> {
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
        Ok(true)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(TrustError::TrustStoreInstallFailed(format!(
            "update-ca-certificates failed: {stderr} — install manually: {manual}"
        )))
    }
}

#[cfg(not(target_os = "linux"))]
fn install_linux(_cert_path: &str, _manual: &str) -> Result<bool, TrustError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn install_command_contains_platform_tool() {
        let cmd = install_command("/tmp/root.cert.pem");
        // Just verify it produces a non-empty command string
        assert!(!cmd.is_empty());
        // On macOS it should contain "security", on Linux "update-ca-certificates"
        #[cfg(target_os = "macos")]
        assert!(cmd.contains("security"));
        #[cfg(target_os = "linux")]
        assert!(cmd.contains("update-ca-certificates"));
    }

    #[test]
    fn install_returns_actionable_error_on_failure() {
        let result = install_root_ca("/nonexistent/path/root.cert.pem");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        // Error should contain actionable instructions
        assert!(
            err_msg.contains("trust")
                || err_msg.contains("install")
                || err_msg.contains("security")
        );
    }
}
