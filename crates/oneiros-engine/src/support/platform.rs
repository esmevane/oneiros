use std::path::PathBuf;

use etcetera::app_strategy::{AppStrategy, AppStrategyArgs, choose_app_strategy};

const TLD: &str = "com";
const AUTHOR: &str = "esmevane";
const APP: &str = "oneiros";

/// Platform-aware path resolution for the application.
///
/// Resolves platform-appropriate paths for data, config, and cache
/// directories using etcetera's XDG/platform strategy. The same
/// identity is used for OS service registration (via `label()`).
///
/// Config uses this for its defaults — CLI args, env vars, and
/// config files can override any of them.
pub(crate) struct Platform {
    data_dir: PathBuf,
    config_dir: PathBuf,
    cache_dir: PathBuf,
}

impl Platform {
    pub(crate) fn resolve() -> Self {
        let args = AppStrategyArgs {
            top_level_domain: TLD.into(),
            author: AUTHOR.into(),
            app_name: APP.into(),
        };

        let strategy = choose_app_strategy(args).expect("unable to determine home directory");

        Self {
            data_dir: strategy.data_dir(),
            config_dir: strategy.config_dir(),
            cache_dir: strategy.cache_dir(),
        }
    }

    /// The application's data directory (e.g., `~/.local/share/oneiros`).
    ///
    /// This is where brain databases, event logs, and tokens live.
    pub(crate) fn data_dir(&self) -> PathBuf {
        self.data_dir.clone()
    }

    /// The application's config directory (e.g., `~/.config/oneiros`).
    pub(crate) fn config_dir(&self) -> PathBuf {
        self.config_dir.clone()
    }

    /// The application's cache directory (e.g., `~/.cache/oneiros`).
    pub(crate) fn cache_dir(&self) -> PathBuf {
        self.cache_dir.clone()
    }

    /// The service label for OS registration (e.g., `com.esmevane.oneiros`).
    pub(crate) fn service_label(&self) -> String {
        format!("{TLD}.{AUTHOR}.{APP}")
    }
}

impl Default for Platform {
    fn default() -> Self {
        Self::resolve()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_dir_ends_with_app_name() {
        let platform = Platform::resolve();
        let data = platform.data_dir();
        assert!(
            data.ends_with(APP),
            "data_dir should end with app name, got: {}",
            data.display()
        );
    }

    #[test]
    fn service_label_is_reverse_domain() {
        let platform = Platform::resolve();
        assert_eq!(platform.service_label(), "com.esmevane.oneiros");
    }
}
