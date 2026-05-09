//! Search filters — limit and offset for bounded index operations.
//!
//! List commands are searches with defaults. These types carry the
//! pagination-without-pagination parameters: how many items to show
//! and where to start in the result set.

use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// Bounded index parameters — flattens into any list request.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct SearchFilters {
    /// Maximum number of items to return
    #[arg(long, default_value = "10")]
    #[serde(default)]
    pub(crate) limit: Limit,

    /// Number of items to skip before returning results
    #[arg(long, default_value = "0")]
    #[serde(default)]
    pub(crate) offset: Offset,
}
