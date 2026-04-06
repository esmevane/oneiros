use bon::Builder;
use lorosurgeon::{Hydrate, Reconcile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(
    Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq, Hydrate, Reconcile,
)]
pub struct Memory {
    #[builder(default)]
    pub id: MemoryId,
    pub agent_id: AgentId,
    #[builder(into)]
    pub level: LevelName,
    #[builder(into)]
    pub content: Content,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

impl Memory {
    /// Produce a compact ref token for this memory (used in dream summaries).
    pub fn ref_token(&self) -> RefToken {
        RefToken::from(Ref::memory(self.id))
    }

    /// Truncate content to the given byte length, appending "…" if truncated.
    pub fn summary(&self, max_len: usize) -> String {
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

#[derive(Hydrate, Reconcile)]
#[loro(root = "memories")]
pub struct Memories(HashMap<String, Memory>);

resource_id!(MemoryId);
