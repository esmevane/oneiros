use oneiros_config::TrustMode;

use crate::ResolvedMode;

/// Resolve the effective TLS mode from config and hostname.
///
/// `TrustMode::Auto` inspects the hostname to pick between `Local` and
/// `Acme`. All other variants map directly to their `ResolvedMode`
/// counterparts.
///
/// # Examples
///
/// ```
/// use oneiros_config::TrustMode;
/// use oneiros_trust::{ResolvedMode, resolve_mode};
///
/// assert_eq!(resolve_mode(&TrustMode::Auto, "localhost"), ResolvedMode::Local);
/// assert_eq!(resolve_mode(&TrustMode::Auto, "brain.example.com"), ResolvedMode::Acme);
/// assert_eq!(resolve_mode(&TrustMode::Off, "brain.example.com"), ResolvedMode::Off);
/// ```
pub fn resolve_mode(configured: &TrustMode, hostname: &str) -> ResolvedMode {
    match configured {
        TrustMode::Local => ResolvedMode::Local,
        TrustMode::Acme => ResolvedMode::Acme,
        TrustMode::Off => ResolvedMode::Off,
        TrustMode::Auto => {
            if is_local_hostname(hostname) {
                ResolvedMode::Local
            } else {
                ResolvedMode::Acme
            }
        }
    }
}

fn is_local_hostname(hostname: &str) -> bool {
    let dominated = hostname.to_lowercase();

    // Exact well-known local names
    if matches!(dominated.as_str(), "localhost" | "127.0.0.1" | "::1" | "0.0.0.0") {
        return true;
    }

    // Local domain suffixes
    if dominated.ends_with(".local")
        || dominated.ends_with(".internal")
        || dominated.ends_with(".home.arpa")
    {
        return true;
    }

    // Private/loopback/link-local IP ranges
    if let Ok(ip) = hostname.parse::<std::net::IpAddr>() {
        return match ip {
            std::net::IpAddr::V4(v4) => v4.is_loopback() || v4.is_private() || v4.is_link_local(),
            std::net::IpAddr::V6(v6) => v6.is_loopback(),
        };
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_localhost_resolves_local() {
        assert_eq!(
            resolve_mode(&TrustMode::Auto, "localhost"),
            ResolvedMode::Local
        );
    }

    #[test]
    fn auto_loopback_resolves_local() {
        assert_eq!(
            resolve_mode(&TrustMode::Auto, "127.0.0.1"),
            ResolvedMode::Local
        );
    }

    #[test]
    fn auto_dot_local_resolves_local() {
        assert_eq!(
            resolve_mode(&TrustMode::Auto, "mybox.local"),
            ResolvedMode::Local
        );
    }

    #[test]
    fn auto_ip_address_resolves_local() {
        assert_eq!(
            resolve_mode(&TrustMode::Auto, "192.168.1.50"),
            ResolvedMode::Local
        );
    }

    #[test]
    fn auto_public_fqdn_resolves_acme() {
        assert_eq!(
            resolve_mode(&TrustMode::Auto, "brain.example.com"),
            ResolvedMode::Acme
        );
    }

    #[test]
    fn explicit_off_resolves_off() {
        assert_eq!(
            resolve_mode(&TrustMode::Off, "brain.example.com"),
            ResolvedMode::Off
        );
    }

    #[test]
    fn explicit_local_overrides_auto() {
        assert_eq!(
            resolve_mode(&TrustMode::Local, "brain.example.com"),
            ResolvedMode::Local
        );
    }

    #[test]
    fn explicit_acme_overrides_auto() {
        assert_eq!(
            resolve_mode(&TrustMode::Acme, "localhost"),
            ResolvedMode::Acme
        );
    }

    #[test]
    fn auto_with_public_ipv4_resolves_acme() {
        assert_eq!(
            resolve_mode(&TrustMode::Auto, "8.8.8.8"),
            ResolvedMode::Acme,
        );
    }

    #[test]
    fn auto_with_public_ipv4_2_resolves_acme() {
        assert_eq!(
            resolve_mode(&TrustMode::Auto, "45.33.32.156"),
            ResolvedMode::Acme,
        );
    }

    #[test]
    fn auto_with_ten_range_resolves_local() {
        assert_eq!(
            resolve_mode(&TrustMode::Auto, "10.0.0.5"),
            ResolvedMode::Local,
        );
    }

    #[test]
    fn auto_with_ipv6_loopback_resolves_local() {
        assert_eq!(
            resolve_mode(&TrustMode::Auto, "::1"),
            ResolvedMode::Local,
        );
    }
}
