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

#[derive(Clone, Default, Hydrate, Reconcile)]
#[loro(root = "memories")]
pub struct Memories(HashMap<String, Memory>);

impl Memories {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, id: MemoryId) -> Option<&Memory> {
        self.0.get(&id.to_string())
    }

    pub fn set(&mut self, memory: &Memory) -> Option<Memory> {
        self.0.insert(memory.id.to_string(), memory.clone())
    }

    pub fn remove(&mut self, memory_id: MemoryId) -> Option<Memory> {
        self.0.remove(&memory_id.to_string())
    }
}

resource_id!(MemoryId);
