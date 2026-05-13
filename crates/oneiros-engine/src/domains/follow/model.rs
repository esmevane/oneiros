use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// A rich join between a local bookmark and a source it's tracking. Follows
/// are created by `bookmark follow`, advanced by `bookmark collect`, and
/// removed by `bookmark unfollow`. Their lifecycle events live on
/// `BookmarkEvents` because follows are bookmark-native — they don't
/// make sense outside a bookmark context.
///
/// `source` is a typed modality (`FollowSource::Local(Ref)` for same-host
/// sources, `FollowSource::Peer(PeerLink)` for cross-host sources). The
/// ticket (when authorization is needed) lives inside the
/// `PeerLink.link` for the Peer variant.
#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Follow {
    #[builder(default)]
    pub(crate) id: FollowId,
    #[builder(into)]
    pub(crate) project: ProjectName,
    #[builder(into)]
    pub(crate) bookmark: BookmarkName,
    pub(crate) source: FollowSource,
    #[builder(default = Checkpoint::empty())]
    pub(crate) checkpoint: Checkpoint,
    #[builder(default = Timestamp::now())]
    pub(crate) created_at: Timestamp,
}

impl Indexable<FollowId> for Follow {
    fn id(&self) -> FollowId {
        self.id
    }
}

pub(crate) type Follows = EntityIndex<FollowId, Follow>;

resource_id!(FollowId);
