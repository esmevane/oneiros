use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

fn default_recent_window() -> usize {
    5
}
fn default_dream_depth() -> usize {
    1
}
fn default_cognition_size() -> usize {
    20
}
fn default_recollection_level() -> LevelName {
    LevelName::new("project")
}
fn default_recollection_size() -> usize {
    30
}
fn default_experience_size() -> usize {
    10
}

/// Configuration for dream assembly — controls BFS traversal depth,
/// size caps, and memory level filtering.
///
/// Carried by `Config` as the server-level default. Per-request callers
/// can supply `DreamOverrides` to selectively replace individual knobs;
/// unspecified overrides inherit the server default.
#[derive(Args, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct DreamConfig {
    /// Number of recent cognitions and experiences to include
    /// in the orientation window.
    #[arg(long, global = true, default_value_t = default_recent_window())]
    #[serde(default = "default_recent_window")]
    pub(crate) recent_window: usize,
    /// Maximum BFS traversal depth from the seed set.
    #[arg(long, global = true, default_value_t = default_dream_depth())]
    #[serde(default = "default_dream_depth")]
    pub(crate) dream_depth: usize,
    /// Maximum number of cognitions in the dream.
    #[arg(long, global = true, default_value_t = default_cognition_size())]
    #[serde(default = "default_cognition_size")]
    pub(crate) cognition_size: usize,
    /// Minimum memory level to include (log-level semantics).
    /// Core memories are always included regardless of this setting.
    #[arg(long, global = true, default_value_t = default_recollection_level())]
    #[serde(default = "default_recollection_level")]
    pub(crate) recollection_level: LevelName,
    /// Maximum number of non-core memories in the dream.
    #[arg(long, global = true, default_value_t = default_recollection_size())]
    #[serde(default = "default_recollection_size")]
    pub(crate) recollection_size: usize,
    /// Maximum number of experiences in the dream.
    #[arg(long, global = true, default_value_t = default_experience_size())]
    #[serde(default = "default_experience_size")]
    pub(crate) experience_size: usize,
}

impl Default for DreamConfig {
    fn default() -> Self {
        Self {
            recent_window: default_recent_window(),
            dream_depth: default_dream_depth(),
            cognition_size: default_cognition_size(),
            recollection_level: default_recollection_level(),
            recollection_size: default_recollection_size(),
            experience_size: default_experience_size(),
        }
    }
}

impl DreamConfig {
    /// Merge per-request overrides onto this config.
    ///
    /// Override fields that are `Some` replace the corresponding default;
    /// `None` fields inherit from self.
    pub(crate) fn merge(&self, overrides: &DreamOverrides) -> Self {
        Self {
            recent_window: overrides.recent_window.unwrap_or(self.recent_window),
            dream_depth: overrides.dream_depth.unwrap_or(self.dream_depth),
            cognition_size: overrides.cognition_size.unwrap_or(self.cognition_size),
            recollection_level: overrides
                .recollection_level
                .clone()
                .unwrap_or(self.recollection_level.clone()),
            recollection_size: overrides
                .recollection_size
                .unwrap_or(self.recollection_size),
            experience_size: overrides.experience_size.unwrap_or(self.experience_size),
        }
    }
}

/// Per-request overrides for dream assembly.
///
/// Every field is optional — only specified fields replace the server
/// default from `DreamConfig`. Arrives via query params, request body,
/// or CLI flags.
#[derive(Debug, Clone, Default, Serialize, Deserialize, schemars::JsonSchema)]
pub(crate) struct DreamOverrides {
    pub(crate) recent_window: Option<usize>,
    pub(crate) dream_depth: Option<usize>,
    pub(crate) cognition_size: Option<usize>,
    pub(crate) recollection_level: Option<LevelName>,
    pub(crate) recollection_size: Option<usize>,
    pub(crate) experience_size: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let config = DreamConfig::default();
        assert_eq!(config.recent_window, 5);
        assert_eq!(config.dream_depth, 1);
        assert_eq!(config.cognition_size, 20);
        assert_eq!(config.recollection_level, LevelName::new("project"));
        assert_eq!(config.recollection_size, 30);
        assert_eq!(config.experience_size, 10);
    }

    #[test]
    fn merge_empty_overrides_preserves_defaults() {
        let config = DreamConfig::default();
        let overrides = DreamOverrides::default();
        let merged = config.merge(&overrides);

        assert_eq!(merged.recent_window, config.recent_window);
        assert_eq!(merged.dream_depth, config.dream_depth);
        assert_eq!(merged.cognition_size, config.cognition_size);
        assert_eq!(merged.recollection_level, config.recollection_level);
        assert_eq!(merged.recollection_size, config.recollection_size);
        assert_eq!(merged.experience_size, config.experience_size);
    }

    #[test]
    fn merge_overrides_replace_selectively() {
        let config = DreamConfig::default();
        let overrides = DreamOverrides {
            recent_window: Some(10),
            cognition_size: Some(50),
            ..Default::default()
        };
        let merged = config.merge(&overrides);

        assert_eq!(merged.recent_window, 10);
        assert_eq!(merged.cognition_size, 50);
        // Unspecified fields inherit from config
        assert_eq!(merged.dream_depth, config.dream_depth);
        assert_eq!(merged.recollection_level, config.recollection_level);
        assert_eq!(merged.recollection_size, config.recollection_size);
        assert_eq!(merged.experience_size, config.experience_size);
    }

    #[test]
    fn merge_can_override_all_fields() {
        let config = DreamConfig::default();
        let overrides = DreamOverrides {
            recent_window: Some(20),
            dream_depth: Some(3),
            cognition_size: Some(100),
            recollection_level: Some(LevelName::new("session")),
            recollection_size: Some(50),
            experience_size: Some(25),
        };
        let merged = config.merge(&overrides);

        assert_eq!(merged.recent_window, 20);
        assert_eq!(merged.dream_depth, 3);
        assert_eq!(merged.cognition_size, 100);
        assert_eq!(merged.recollection_level, LevelName::new("session"));
        assert_eq!(merged.recollection_size, 50);
        assert_eq!(merged.experience_size, 25);
    }
}
