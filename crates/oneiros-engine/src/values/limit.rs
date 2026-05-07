use rusqlite::types::{ToSql, ToSqlOutput};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// How many items to return. Defaults to 10 — small enough not to flood
/// CLI output or MCP context; agents who want more drill in with
/// progressive commands.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(transparent)]
pub(crate) struct Limit(pub(crate) usize);

impl Default for Limit {
    fn default() -> Self {
        Self(10)
    }
}

impl core::fmt::Display for Limit {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl core::str::FromStr for Limit {
    type Err = core::num::ParseIntError;

    fn from_str(given_str: &str) -> Result<Self, Self::Err> {
        Ok(Self(given_str.parse()?))
    }
}

impl ToSql for Limit {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(
            self.0 as i64,
        )))
    }
}
