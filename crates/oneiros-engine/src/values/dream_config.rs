use serde::{Deserialize, Serialize};

use crate::*;

/// Configuration for dream assembly — controls BFS traversal depth,
/// size caps, and memory level filtering.
///
/// Carried by `Config` as the server-level default. Per-request callers
/// can supply `DreamOverrides` to selectively replace individual knobs;
/// unspecified overrides inherit the server default.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct DreamConfig {
    /// Number of recent cognitions and experiences to include
    /// in the orientation window.
    pub recent_window: usize,
    /// Maximum BFS traversal depth from the seed set.
    /// None means unlimited.
    pub dream_depth: Option<usize>,
    /// Maximum number of cognitions in the dream.
    /// None means unlimited.
    pub cognition_size: Option<usize>,
    /// Minimum memory level to include (log-level semantics).
    /// Core memories are always included regardless of this setting.
    /// None means include all levels.
    pub recollection_level: Option<LevelName>,
    /// Maximum number of non-core memories in the dream.
    /// None means unlimited.
    pub recollection_size: Option<usize>,
    /// Maximum number of experiences in the dream.
    /// None means unlimited.
    pub experience_size: Option<usize>,
}

impl Default for DreamConfig {
    fn default() -> Self {
        Self {
            recent_window: 5,
            dream_depth: Some(1),
            cognition_size: Some(20),
            recollection_level: Some(LevelName::new("project")),
            recollection_size: Some(30),
            experience_size: Some(10),
        }
    }
}

impl DreamConfig {
    /// Merge per-request overrides onto this config.
    ///
    /// Override fields that are `Some` replace the corresponding default;
    /// `None` fields inherit from self.
    pub fn merge(&self, overrides: &DreamOverrides) -> Self {
        Self {
            recent_window: overrides.recent_window.unwrap_or(self.recent_window),
            dream_depth: overrides.dream_depth.or(self.dream_depth),
            cognition_size: overrides.cognition_size.or(self.cognition_size),
            recollection_level: overrides
                .recollection_level
                .clone()
                .or(self.recollection_level.clone()),
            recollection_size: overrides.recollection_size.or(self.recollection_size),
            experience_size: overrides.experience_size.or(self.experience_size),
        }
    }
}

/// Per-request overrides for dream assembly.
///
/// Every field is optional — only specified fields replace the server
/// default from `DreamConfig`. Arrives via query params, request body,
/// or CLI flags.
#[derive(Debug, Clone, Default, Serialize, Deserialize, schemars::JsonSchema)]
pub struct DreamOverrides {
    pub recent_window: Option<usize>,
    pub dream_depth: Option<usize>,
    pub cognition_size: Option<usize>,
    pub recollection_level: Option<LevelName>,
    pub recollection_size: Option<usize>,
    pub experience_size: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let config = DreamConfig::default();
        assert_eq!(config.recent_window, 5);
        assert_eq!(config.dream_depth, Some(1));
        assert_eq!(config.cognition_size, Some(20));
        assert_eq!(config.recollection_level, Some(LevelName::new("project")));
        assert_eq!(config.recollection_size, Some(30));
        assert_eq!(config.experience_size, Some(10));
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
        assert_eq!(merged.cognition_size, Some(50));
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
        assert_eq!(merged.dream_depth, Some(3));
        assert_eq!(merged.cognition_size, Some(100));
        assert_eq!(merged.recollection_level, Some(LevelName::new("session")));
        assert_eq!(merged.recollection_size, Some(50));
        assert_eq!(merged.experience_size, Some(25));
    }
}
