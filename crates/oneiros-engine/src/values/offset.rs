use rusqlite::types::{ToSql, ToSqlOutput};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};

/// Where to begin in the result set. Defaults to 0.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(transparent)]
pub(crate) struct Offset(pub(crate) usize);

// Hand-written so query-string values (always strings on the wire) deserialize
// as well as JSON numeric values. The derived transparent impl only accepts
// u64, which silently fails inside untagged enum aggregation.
impl<'de> Deserialize<'de> for Offset {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de;

        struct OffsetVisitor;
        impl<'de> de::Visitor<'de> for OffsetVisitor {
            type Value = Offset;
            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a number or numeric string")
            }
            fn visit_u64<E: de::Error>(self, value: u64) -> Result<Offset, E> {
                Ok(Offset(value as usize))
            }
            fn visit_str<E: de::Error>(self, value: &str) -> Result<Offset, E> {
                value.parse::<usize>().map(Offset).map_err(de::Error::custom)
            }
        }
        deserializer.deserialize_any(OffsetVisitor)
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
