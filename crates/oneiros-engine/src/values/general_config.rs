use bon::Builder;
use serde::{Deserialize, Serialize};

/// General system configuration — settings that don't belong to any
/// specific domain.
///
/// Lives inside [`Config`] as the `[general]` section. Currently holds
/// the default page size for list/pagination across all endpoints.
/// Will grow as more cross-cutting knobs are identified.
#[derive(Builder, Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct GeneralConfig {
    /// Default page size for paginated list responses. Used when no
    /// explicit `limit` parameter is provided. Defaults to 10.
    #[builder(default = 10usize)]
    pub(crate) default_page_size: usize,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}
