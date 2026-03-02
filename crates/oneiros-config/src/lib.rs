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
}
