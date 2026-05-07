use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Memory {
    #[builder(default)]
    pub(crate) id: MemoryId,
    pub(crate) agent_id: AgentId,
    #[builder(into)]
    pub(crate) level: LevelName,
    #[builder(into)]
    pub(crate) content: Content,
    #[builder(default = Timestamp::now())]
    pub(crate) created_at: Timestamp,
}

impl Memory {
    /// Produce a compact ref token for this memory (used in dream summaries).
    pub(crate) fn ref_token(&self) -> RefToken {
        RefToken::from(Ref::memory(self.id))
    }

    /// Truncate content to the given byte length, appending "…" if truncated.
    pub(crate) fn summary(&self, max_len: usize) -> String {
        let s = self.content.as_str();
        if s.len() <= max_len {
            s.to_string()
        } else {
            let mut end = max_len;
            while end > 0 && !s.is_char_boundary(end) {
                end -= 1;
            }
            format!("{}…", &s[..end])
        }
    }
}

impl Indexable<MemoryId> for Memory {
    fn id(&self) -> MemoryId {
        self.id
    }
}

pub(crate) type Memories = EntityIndex<MemoryId, Memory>;

resource_id!(MemoryId);
