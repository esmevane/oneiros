use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// A known remote host — a project on another host you can push bookmarks
/// to and pull bookmarks from, authorized by a project-scoped ticket.
#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Remote {
    #[builder(default)]
    pub(crate) id: RemoteId,
    pub(crate) name: RemoteName,
    pub(crate) address: PeerAddress,
    /// The project-scoped capability ticket.
    pub(crate) ticket: Link,
    /// Which project on the remote this points to.
    #[builder(into)]
    pub(crate) project: ProjectName,
    #[builder(default = Timestamp::now())]
    pub(crate) created_at: Timestamp,
}

impl Indexable<RemoteId> for Remote {
    fn id(&self) -> RemoteId {
        self.id
    }
}

pub(crate) type Remotes = EntityIndex<RemoteId, Remote>;

resource_id!(RemoteId);
resource_name!(RemoteName);
