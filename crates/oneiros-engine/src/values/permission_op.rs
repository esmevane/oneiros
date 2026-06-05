use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A capability operation that a ticket holder may perform.
///
/// Permissions are carried on [`Ticket`](crate::Ticket) via the versioned
/// [`Permission`](crate::Permission) wrapper. The operation describes *what*
/// the holder is allowed to do; the ticket's [`Link::target`](crate::Link)
/// describes *to what resource*.
///
/// For v1, the operations are:
///
/// - `Read`  — pull, diff, fetch, list bookmarks
/// - `Write` — submit, push bookmarks
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::ValueEnum,
)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum PermissionOp {
    Read,
    Write,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::ValueEnum;
    use pretty_assertions::assert_eq;

    #[test]
    fn value_enum_names_are_kebab_cased() {
        assert_eq!(
            PermissionOp::Read.to_possible_value().unwrap().get_name(),
            "read"
        );
        assert_eq!(
            PermissionOp::Write.to_possible_value().unwrap().get_name(),
            "write"
        );
    }

    #[test]
    fn serde_roundtrip_read() {
        let json = serde_json::json!("read");
        let decoded: PermissionOp = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(decoded, PermissionOp::Read);
        let encoded = serde_json::to_value(&decoded).unwrap();
        assert_eq!(encoded, json);
    }

    #[test]
    fn serde_roundtrip_write() {
        let json = serde_json::json!("write");
        let decoded: PermissionOp = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(decoded, PermissionOp::Write);
        let encoded = serde_json::to_value(&decoded).unwrap();
        assert_eq!(encoded, json);
    }
}
