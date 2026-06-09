use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A capability operation that a ticket holder may perform.
///
/// These map to the bridge operations a remote ticket authorizes:
///
/// - `BookmarkPush` — push a bookmark to the remote (BridgePushBookmark)
/// - `BookmarkList` — list bookmarks on the remote (BridgeListBookmarks)
///
/// Bookmark collection from a remote uses the existing chronicle diff
/// protocol (BridgeDiff + BridgeFetchEvents) rather than a dedicated
/// bridge operation, so no separate permission is needed.
///
/// When a ticket's `permissions` vec is empty (V0, all existing tickets),
/// all operations are granted — the ticket IS the permission.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::ValueEnum,
)]
#[serde(rename_all = "kebab-case")]
#[allow(clippy::enum_variant_names)]
pub(crate) enum PermissionOp {
    BookmarkPush,
    BookmarkList,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::ValueEnum;
    use pretty_assertions::assert_eq;

    #[test]
    fn value_enum_names_are_kebab_cased() {
        assert_eq!(
            PermissionOp::BookmarkPush
                .to_possible_value()
                .unwrap()
                .get_name(),
            "bookmark-push"
        );
        assert_eq!(
            PermissionOp::BookmarkList
                .to_possible_value()
                .unwrap()
                .get_name(),
            "bookmark-list"
        );
    }

    #[test]
    fn serde_roundtrip_push() {
        let json = serde_json::json!("bookmark-push");
        let decoded: PermissionOp = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(decoded, PermissionOp::BookmarkPush);
        let encoded = serde_json::to_value(decoded).unwrap();
        assert_eq!(encoded, json);
    }

    #[test]
    fn serde_roundtrip_list() {
        let json = serde_json::json!("bookmark-list");
        let decoded: PermissionOp = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(decoded, PermissionOp::BookmarkList);
        let encoded = serde_json::to_value(decoded).unwrap();
        assert_eq!(encoded, json);
    }
}
