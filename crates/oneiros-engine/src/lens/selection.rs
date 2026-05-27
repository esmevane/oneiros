use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", content = "data", rename_all = "kebab-case")]
pub(crate) enum Hit {
    Event(EventHit),
    Entity(EntityHit),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub(crate) struct EventHit {
    pub(crate) event_id: EventId,
    pub(crate) timestamp: Timestamp,
    pub(crate) relevance: Relevance,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub(crate) struct EntityHit {
    pub(crate) entity_ref: Ref,
    pub(crate) timestamp: Timestamp,
    pub(crate) relevance: Relevance,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub(crate) enum Relevance {
    Known { score: f64 },
    Unknown,
}

impl Hit {
    pub(crate) fn identity(&self) -> HitIdentity {
        match self {
            Hit::Event(e) => HitIdentity::Event(e.event_id),
            Hit::Entity(e) => HitIdentity::Entity(e.entity_ref.clone()),
        }
    }

    pub(crate) fn timestamp(&self) -> Timestamp {
        match self {
            Hit::Event(e) => e.timestamp,
            Hit::Entity(e) => e.timestamp,
        }
    }

    pub(crate) fn relevance(&self) -> &Relevance {
        match self {
            Hit::Event(e) => &e.relevance,
            Hit::Entity(e) => &e.relevance,
        }
    }

    fn merge(&self, other: &Self) -> Self {
        let merged_relevance = self.relevance().merge(other.relevance());
        let earlier_timestamp = self.timestamp().min(other.timestamp());
        match self {
            Hit::Event(e) => Hit::Event(EventHit {
                event_id: e.event_id,
                timestamp: earlier_timestamp,
                relevance: merged_relevance,
            }),
            Hit::Entity(e) => Hit::Entity(EntityHit {
                entity_ref: e.entity_ref.clone(),
                timestamp: earlier_timestamp,
                relevance: merged_relevance,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum HitIdentity {
    Event(EventId),
    Entity(Ref),
}

impl Relevance {
    pub(crate) fn score(&self) -> Option<f64> {
        match self {
            Relevance::Known { score } => Some(*score),
            Relevance::Unknown => None,
        }
    }

    fn merge(&self, other: &Self) -> Self {
        match (self.score(), other.score()) {
            (Some(a), Some(b)) => Relevance::Known { score: a.max(b) },
            (Some(_), None) => self.clone(),
            (None, Some(_)) => other.clone(),
            (None, None) => Relevance::Unknown,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Selection {
    entries: HashMap<HitIdentity, Hit>,
}

impl Selection {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn insert(&mut self, hit: Hit) {
        let identity = hit.identity();
        self.entries
            .entry(identity)
            .and_modify(|existing| *existing = existing.merge(&hit))
            .or_insert(hit);
    }

    pub(crate) fn union(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for hit in other.entries.values() {
            result.insert(hit.clone());
        }
        result
    }

    pub(crate) fn intersect(&self, other: &Self) -> Self {
        let mut result = Selection::new();
        for (identity, hit) in &self.entries {
            if let Some(other_hit) = other.entries.get(identity) {
                result
                    .entries
                    .insert(identity.clone(), hit.merge(other_hit));
            }
        }
        result
    }

    pub(crate) fn difference(&self, other: &Self) -> Self {
        let mut result = Selection::new();
        for (identity, hit) in &self.entries {
            if !other.entries.contains_key(identity) {
                result.entries.insert(identity.clone(), hit.clone());
            }
        }
        result
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }

    #[allow(dead_code)]
    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub(crate) fn sorted_by_timestamp_desc(self) -> Vec<Hit> {
        let mut hits: Vec<Hit> = self.entries.into_values().collect();
        hits.sort_by_key(|h| std::cmp::Reverse(h.timestamp()));
        hits
    }
}

impl IntoIterator for Selection {
    type Item = Hit;
    type IntoIter = std::collections::hash_map::IntoValues<HitIdentity, Hit>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entity_hit(name: &str, score: Option<f64>) -> Hit {
        let ref_ = Ref::agent(AgentId::new());
        let _ = name;
        Hit::Entity(EntityHit {
            entity_ref: ref_,
            timestamp: Timestamp::now(),
            relevance: match score {
                Some(s) => Relevance::Known { score: s },
                None => Relevance::Unknown,
            },
        })
    }

    fn selection_of(hits: Vec<Hit>) -> Selection {
        let mut sel = Selection::new();
        for hit in hits {
            sel.insert(hit);
        }
        sel
    }

    fn shared_entity_hit(ref_: &Ref, score: Option<f64>) -> Hit {
        Hit::Entity(EntityHit {
            entity_ref: ref_.clone(),
            timestamp: Timestamp::now(),
            relevance: match score {
                Some(s) => Relevance::Known { score: s },
                None => Relevance::Unknown,
            },
        })
    }

    #[test]
    fn union_is_commutative() {
        let a = entity_hit("a", Some(1.0));
        let b = entity_hit("b", Some(2.0));
        let set_a = selection_of(vec![a.clone()]);
        let set_b = selection_of(vec![b.clone()]);

        let ab = set_a.union(&set_b);
        let ba = set_b.union(&set_a);
        assert_eq!(ab.len(), ba.len());
        assert_eq!(ab.len(), 2);
    }

    #[test]
    fn intersect_is_commutative() {
        let shared_ref = Ref::agent(AgentId::new());
        let a = shared_entity_hit(&shared_ref, Some(1.0));
        let b = shared_entity_hit(&shared_ref, Some(2.0));
        let extra = entity_hit("extra", None);

        let set_a = selection_of(vec![a.clone(), extra.clone()]);
        let set_b = selection_of(vec![b.clone()]);

        let ab = set_a.intersect(&set_b);
        let ba = set_b.intersect(&set_a);
        assert_eq!(ab.len(), ba.len());
        assert_eq!(ab.len(), 1);
    }

    #[test]
    fn union_identity_with_empty() {
        let a = entity_hit("a", Some(1.0));
        let set_a = selection_of(vec![a]);
        let empty = Selection::new();

        let result = set_a.union(&empty);
        assert_eq!(result.len(), set_a.len());
    }

    #[test]
    fn union_is_idempotent() {
        let a = entity_hit("a", Some(1.0));
        let set_a = selection_of(vec![a]);

        let result = set_a.union(&set_a);
        assert_eq!(result.len(), set_a.len());
    }

    #[test]
    fn intersect_with_empty_is_empty() {
        let a = entity_hit("a", Some(1.0));
        let set_a = selection_of(vec![a]);
        let empty = Selection::new();

        let result = set_a.intersect(&empty);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn difference_with_empty_is_identity() {
        let a = entity_hit("a", Some(1.0));
        let set_a = selection_of(vec![a]);
        let empty = Selection::new();

        let result = set_a.difference(&empty);
        assert_eq!(result.len(), set_a.len());
    }

    #[test]
    fn difference_with_self_is_empty() {
        let a = entity_hit("a", Some(1.0));
        let set_a = selection_of(vec![a]);

        let result = set_a.difference(&set_a);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn union_merges_relevance_by_max() {
        let shared_ref = Ref::agent(AgentId::new());
        let low = shared_entity_hit(&shared_ref, Some(1.0));
        let high = shared_entity_hit(&shared_ref, Some(5.0));

        let set_a = selection_of(vec![low]);
        let set_b = selection_of(vec![high]);

        let result = set_a.union(&set_b);
        assert_eq!(result.len(), 1);
        let hit = result.into_iter().next().unwrap();
        assert_eq!(hit.relevance().score(), Some(5.0));
    }

    #[test]
    fn intersect_merges_relevance_by_max() {
        let shared_ref = Ref::agent(AgentId::new());
        let low = shared_entity_hit(&shared_ref, Some(1.0));
        let high = shared_entity_hit(&shared_ref, Some(5.0));

        let set_a = selection_of(vec![low]);
        let set_b = selection_of(vec![high]);

        let result = set_a.intersect(&set_b);
        assert_eq!(result.len(), 1);
        let hit = result.into_iter().next().unwrap();
        assert_eq!(hit.relevance().score(), Some(5.0));
    }
}
