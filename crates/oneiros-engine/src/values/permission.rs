use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// A versioned permission — the capability a ticket grants its holder.
///
/// Uses `#[serde(untagged)]` so `{}` deserializes to `V0` (empty struct,
/// implicit read access) and `{"operation": "read"}` deserializes to `V1`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub(crate) enum Permission {
    V1(PermissionV1),
    V0(PermissionV0),
}

/// V1: an explicit operation. This is the latest version.
#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub(crate) struct PermissionV1 {
    pub(crate) operation: PermissionOp,
}

/// V0: implicit read access — the pre-permissions behavior.
#[derive(Debug, Clone, Default, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub(crate) struct PermissionV0 {}

impl Permission {
    #[allow(dead_code)]
    pub(crate) fn builder_v1() -> PermissionV1Builder {
        PermissionV1::builder()
    }

    #[allow(dead_code)]
    pub(crate) fn current(&self) -> Result<PermissionV1, UpcastError> {
        Ok(match self {
            Self::V1(v) => v.clone(),
            Self::V0(v) => v.clone().try_into()?,
        })
    }
}

impl From<PermissionV1> for Permission {
    fn from(v: PermissionV1) -> Self {
        Self::V1(v)
    }
}

impl From<PermissionV0> for Permission {
    fn from(v: PermissionV0) -> Self {
        Self::V0(v)
    }
}

/// V0 → V1 upcast: an empty V0 permission implies read access.
impl TryFrom<PermissionV0> for PermissionV1 {
    type Error = UpcastError;

    fn try_from(_: PermissionV0) -> Result<Self, Self::Error> {
        Ok(PermissionV1 {
            operation: PermissionOp::Read,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn v0_serde_roundtrip_empty_object() {
        let json = serde_json::json!({});
        let decoded: Permission = serde_json::from_value(json.clone()).unwrap();
        assert!(matches!(decoded, Permission::V0(_)));
        let encoded = serde_json::to_value(&decoded).unwrap();
        assert_eq!(encoded, json);
    }

    #[test]
    fn v1_read_serde_roundtrip() {
        let json = serde_json::json!({"operation": "read"});
        let decoded: Permission = serde_json::from_value(json.clone()).unwrap();
        assert!(matches!(decoded, Permission::V1(_)));
        let v1 = decoded.current().unwrap();
        assert_eq!(v1.operation, PermissionOp::Read);
        let encoded = serde_json::to_value(&decoded).unwrap();
        assert_eq!(encoded, json);
    }

    #[test]
    fn v1_write_serde_roundtrip() {
        let json = serde_json::json!({"operation": "write"});
        let decoded: Permission = serde_json::from_value(json.clone()).unwrap();
        assert!(matches!(decoded, Permission::V1(_)));
        let v1 = decoded.current().unwrap();
        assert_eq!(v1.operation, PermissionOp::Write);
        let encoded = serde_json::to_value(&decoded).unwrap();
        assert_eq!(encoded, json);
    }

    #[test]
    fn v0_upcasts_to_read() {
        let perm = Permission::V0(PermissionV0 {});
        let v1 = perm.current().unwrap();
        assert_eq!(v1.operation, PermissionOp::Read);
    }
}
