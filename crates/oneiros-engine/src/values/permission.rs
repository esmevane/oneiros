use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// A versioned permission — the capability a ticket grants its holder.
///
/// Uses `#[serde(untagged)]` so `{}` deserializes to `V0` (empty struct,
/// implicit read access) and `{"operation": "read"}` deserializes to `V1`.
///
/// When a ticket's `permissions` vector is empty, the field is skipped
/// entirely via `#[serde(skip_serializing_if)]`. Existing tickets (no
/// permissions key) deserialize to an empty vec.
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
    /// Build the latest version directly.
    #[allow(dead_code)]
    pub(crate) fn builder_v1() -> PermissionV1Builder {
        PermissionV1::builder()
    }

    /// Upcast to the latest version by reference.
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

    #[test]
    fn empty_vec_serializes_as_absent() {
        // Ticket's #[serde(skip_serializing_if = "Vec::is_empty")] means
        // an empty permissions vec won't appear in JSON output.
        let ticket = Ticket::builder()
            .actor_id(ActorId::new())
            .project_name(ProjectName::new("test"))
            .project_id(ProjectId::new())
            .link(Link::new(
                Ref::project(ProjectId::new()),
                Token::from("tok"),
            ))
            .granted_by(ActorId::new())
            .build();
        // Default builder leaves permissions empty.
        assert!(ticket.permissions.is_empty());
        let json = serde_json::to_value(&ticket).unwrap();
        assert!(
            json.get("permissions").is_none(),
            "empty permissions should be absent from JSON"
        );
    }

    #[test]
    fn absent_permissions_deserializes_to_empty() {
        let json = serde_json::json!({
            "id": "019aaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
            "actor_id": "019aaaaa-bbbb-cccc-dddd-ffffffffffff",
            "project_name": "test",
            "project_id": "019aaaaa-bbbb-cccc-dddd-000000000001",
            "link": {"target": {"V0": {"Project": "019aaaaa-bbbb-cccc-dddd-000000000001"}}, "token": "tok"},
            "granted_by": "019aaaaa-bbbb-cccc-dddd-ffffffffffff",
            "uses": 0,
            "created_at": "2026-06-05T00:00:00Z"
        });
        let ticket: Ticket = serde_json::from_value(json).unwrap();
        assert!(ticket.permissions.is_empty());
    }
}
