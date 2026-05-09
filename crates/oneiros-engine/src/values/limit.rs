use rusqlite::types::{ToSql, ToSqlOutput};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};

/// How many items to return. Defaults to 10 — small enough not to flood
/// CLI output or MCP context; agents who want more drill in with
/// progressive commands.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(transparent)]
pub(crate) struct Limit(pub(crate) usize);

// Hand-written so query-string values (always strings on the wire) deserialize
// as well as JSON numeric values. The derived transparent impl only accepts
// u64, which silently fails inside untagged enum aggregation.
impl<'de> Deserialize<'de> for Limit {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de;

        struct LimitVisitor;
        impl<'de> de::Visitor<'de> for LimitVisitor {
            type Value = Limit;
            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a number or numeric string")
            }
            fn visit_u64<E: de::Error>(self, value: u64) -> Result<Limit, E> {
                Ok(Limit(value as usize))
            }
            fn visit_str<E: de::Error>(self, value: &str) -> Result<Limit, E> {
                value.parse::<usize>().map(Limit).map_err(de::Error::custom)
            }
        }
        deserializer.deserialize_any(LimitVisitor)
    }
}

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
