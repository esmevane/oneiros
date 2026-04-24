//! Search filters — limit and offset for bounded index operations.
//!
//! List commands are searches with defaults. These types carry the
//! pagination-without-pagination parameters: how many items to show
//! and where to start in the result set.

use clap::Args;
use rusqlite::types::{ToSql, ToSqlOutput};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};

/// How many items to return. Defaults to 10 — small enough not to flood
/// CLI output or MCP context; agents who want more drill in with
/// progressive commands.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(transparent)]
pub struct Limit(pub usize);

impl<'de> Deserialize<'de> for Limit {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de;

        struct LimitVisitor;
        impl<'de> de::Visitor<'de> for LimitVisitor {
            type Value = Limit;
            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str("a number or numeric string")
            }
            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Limit, E> {
                Ok(Limit(v as usize))
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<Limit, E> {
                v.parse::<usize>().map(Limit).map_err(de::Error::custom)
            }
        }
        deserializer.deserialize_any(LimitVisitor)
    }
}

impl Limit {
    pub fn new(value: usize) -> Self {
        Self(value)
    }
}

impl Default for Limit {
    fn default() -> Self {
        Self(10)
    }
}

impl core::fmt::Display for Limit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl core::str::FromStr for Limit {
    type Err = core::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl ToSql for Limit {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(
            self.0 as i64,
        )))
    }
}

/// Where to begin in the result set. Defaults to 0.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(transparent)]
pub struct Offset(pub usize);

impl<'de> Deserialize<'de> for Offset {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de;

        struct OffsetVisitor;
        impl<'de> de::Visitor<'de> for OffsetVisitor {
            type Value = Offset;
            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str("a number or numeric string")
            }
            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Offset, E> {
                Ok(Offset(v as usize))
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<Offset, E> {
                v.parse::<usize>().map(Offset).map_err(de::Error::custom)
            }
        }
        deserializer.deserialize_any(OffsetVisitor)
    }
}

impl Offset {
    pub fn new(value: usize) -> Self {
        Self(value)
    }
}

impl core::fmt::Display for Offset {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl core::str::FromStr for Offset {
    type Err = core::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl ToSql for Offset {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(
            self.0 as i64,
        )))
    }
}

/// Bounded index parameters — flattens into any list request.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Args)]
pub struct SearchFilters {
    /// Maximum number of items to return
    #[arg(long, default_value = "10")]
    #[serde(default)]
    pub limit: Limit,

    /// Number of items to skip before returning results
    #[arg(long, default_value = "0")]
    #[serde(default)]
    pub offset: Offset,
}
