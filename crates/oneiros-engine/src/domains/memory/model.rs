use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum Memory {
    Current(MemoryV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct MemoryV1 {
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

impl MemoryV1 {
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

impl Memory {
    pub fn build_v1() -> MemoryV1Builder {
        MemoryV1::builder()
    }

    pub fn id(&self) -> MemoryId {
        match self {
            Self::Current(v) => v.id,
        }
    }

    pub fn agent_id(&self) -> AgentId {
        match self {
            Self::Current(v) => v.agent_id,
        }
    }

    pub fn level(&self) -> &LevelName {
        match self {
            Self::Current(v) => &v.level,
        }
    }

    pub fn content(&self) -> &Content {
        match self {
            Self::Current(v) => &v.content,
        }
    }

    pub fn created_at(&self) -> Timestamp {
        match self {
            Self::Current(v) => v.created_at,
        }
    }

    /// Produce a compact ref token for this memory (used in dream summaries).
    pub fn ref_token(&self) -> RefToken {
        match self {
            Self::Current(v) => v.ref_token(),
        }
    }

    /// Truncate content to the given byte length, appending "…" if truncated.
    pub fn summary(&self, max_len: usize) -> String {
        match self {
            Self::Current(v) => v.summary(max_len),
        }
    }
}

#[derive(Clone, Default)]
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
        self.0.insert(memory.id().to_string(), memory.clone())
    }

    pub fn values(&self) -> impl Iterator<Item = &Memory> {
        self.0.values()
    }

    pub fn remove(&mut self, memory_id: MemoryId) -> Option<Memory> {
        self.0.remove(&memory_id.to_string())
    }
}

resource_id!(MemoryId);
