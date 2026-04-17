use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
pub struct Follow {
    #[builder(default)]
    pub id: FollowId,
    #[builder(into)]
    pub brain: BrainName,
    #[builder(into)]
    pub bookmark: BookmarkName,
    pub source: FollowSource,
    #[builder(default = Checkpoint::empty())]
    pub checkpoint: Checkpoint,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

#[derive(Clone, Default)]
pub struct Follows(HashMap<String, Follow>);

impl Follows {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn values(&self) -> impl Iterator<Item = &Follow> {
        self.0.values()
    }

    pub fn get(&self, id: FollowId) -> Option<&Follow> {
        self.0.get(&id.to_string())
    }

    pub fn set(&mut self, follow: &Follow) -> Option<Follow> {
        self.0.insert(follow.id.to_string(), follow.clone())
    }

    pub fn remove(&mut self, follow_id: FollowId) -> Option<Follow> {
        self.0.remove(&follow_id.to_string())
    }
}

resource_id!(FollowId);
