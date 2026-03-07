use std::net::SocketAddr;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Read(#[from] std::io::Error),
    #[error("Failed to parse config file: {0}")]
    Parse(#[from] toml::de::Error),
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Config {
    pub service: ServiceConfig,
    pub trust: TrustConfig,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TrustMode {
    Auto,
    Local,
    Acme,
    #[default]
    Off,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TrustConfig {
    pub mode: TrustMode,
    pub acme: AcmeConfig,
    pub peers: Vec<PeerConfig>,
    pub insecure: Vec<InsecureConfig>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AcmeConfig {
    pub contact: Option<String>,
    pub directory: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PeerConfig {
    pub endpoint: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InsecureConfig {
    pub endpoint: String,
    pub reason: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ServiceConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 2100,
        }
    }
}

impl Config {
    /// Load configuration from a TOML file, falling back to defaults if
    /// the file is missing or empty.
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        match std::fs::read_to_string(path) {
            Ok(contents) if !contents.trim().is_empty() => Ok(toml::from_str(&contents)?),
            Ok(_) => Ok(Self::default()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(error) => Err(ConfigError::Read(error)),
        }
    }

    pub fn service_addr(&self) -> SocketAddr {
        self.service.addr()
    }
}

impl ServiceConfig {
    pub fn addr(&self) -> SocketAddr {
        use std::net::ToSocketAddrs;
        format!("{}:{}", self.host, self.port)
            .to_socket_addrs()
            .expect("valid service address")
            .next()
            .expect("at least one resolved address")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_expected_values() {
        let config = Config::default();
        assert_eq!(config.service.host, "127.0.0.1");
        assert_eq!(config.service.port, 2100);
    }

    #[test]
    fn service_addr_resolves() {
        let config = Config::default();
        let addr = config.service_addr();
        assert_eq!(addr.port(), 2100);
        assert_eq!(addr.ip().to_string(), "127.0.0.1");
    }

    #[test]
    fn load_missing_file_returns_defaults() {
        let config = Config::load(Path::new("/nonexistent/config.toml")).unwrap();
        assert_eq!(config.service.port, 2100);
    }

    #[test]
    fn load_empty_file_returns_defaults() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join("config.toml");
        std::fs::write(&path, "").unwrap();
        let config = Config::load(&path).unwrap();
        assert_eq!(config.service.port, 2100);
    }

    #[test]
    fn load_partial_config_fills_defaults() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join("config.toml");
        std::fs::write(&path, "[service]\nport = 3000\n").unwrap();
        let config = Config::load(&path).unwrap();
        assert_eq!(config.service.port, 3000);
        assert_eq!(config.service.host, "127.0.0.1");
    }

    #[test]
    fn roundtrip_toml_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.service.port, config.service.port);
        assert_eq!(parsed.service.host, config.service.host);
    }

    #[test]
    fn trust_config_defaults() {
        let config = Config::default();
        assert_eq!(config.trust.mode, TrustMode::Off);
        assert!(config.trust.acme.contact.is_none());
        assert!(config.trust.peers.is_empty());
        assert!(config.trust.insecure.is_empty());
    }

    #[test]
    fn trust_config_from_toml() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join("config.toml");
        std::fs::write(
            &path,
            r#"
[trust]
mode = "local"

[trust.acme]
contact = "mailto:test@example.com"

[[trust.peers]]
endpoint = "192.168.1.50:2100"
fingerprint = "sha256:abc123"

[[trust.insecure]]
endpoint = "10.0.0.5:2100"
reason = "Test node"
"#,
        )
        .unwrap();
        let config = Config::load(&path).unwrap();
        assert_eq!(config.trust.mode, TrustMode::Local);
        assert_eq!(
            config.trust.acme.contact.as_deref(),
            Some("mailto:test@example.com")
        );
        assert_eq!(config.trust.peers.len(), 1);
        assert_eq!(config.trust.insecure.len(), 1);
    }
}
