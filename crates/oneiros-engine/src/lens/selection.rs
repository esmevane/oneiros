use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", content = "data", rename_all = "kebab-case")]
pub(crate) enum Hit {
    Event(EventHit),
    Entity(EntityHit),
    Name(NameHit),
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
pub(crate) struct NameHit {
    pub(crate) name: String,
    pub(crate) kind: NameKind,
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
            Hit::Name(n) => HitIdentity::Name {
                name: n.name.clone(),
                kind: n.kind,
            },
        }
    }

    pub(crate) fn timestamp(&self) -> Timestamp {
        match self {
            Hit::Event(e) => e.timestamp,
            Hit::Entity(e) => e.timestamp,
            Hit::Name(n) => n.timestamp,
        }
    }

    pub(crate) fn relevance(&self) -> &Relevance {
        match self {
            Hit::Event(e) => &e.relevance,
            Hit::Entity(e) => &e.relevance,
            Hit::Name(n) => &n.relevance,
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
            Hit::Name(n) => Hit::Name(NameHit {
                name: n.name.clone(),
                kind: n.kind,
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
    Name { name: String, kind: NameKind },
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

    /// Returns a new [`Selection`] containing only the hits that satisfy
    /// the predicate. Order is not preserved — selections are sets.
    #[allow(dead_code)]
    pub(crate) fn filter(mut self, predicate: impl Fn(&Hit) -> bool) -> Self {
        self.entries.retain(|_, hit| predicate(hit));
        self
    }

    /// Sorts hits by timestamp descending, skips `offset` entries, and
    /// returns at most `limit` hits. The result is a plain [`Vec`] —
    /// pagination is a terminal operation.
    pub(crate) fn paginate(self, offset: usize, limit: usize) -> Vec<Hit> {
        self.sorted_by_timestamp_desc()
            .into_iter()
            .skip(offset)
            .take(limit)
            .collect()
    }

    /// Collects every [`Ref`] from entity hits in this selection.
    pub(crate) fn entity_refs(&self) -> Vec<Ref> {
        let mut refs = Vec::new();
        for hit in self.entries.values() {
            if let Hit::Entity(EntityHit { entity_ref, .. }) = hit {
                refs.push(entity_ref.clone());
            }
        }
        refs
    }

    /// Collects every [`EventId`] from event hits in this selection.
    pub(crate) fn event_ids(&self) -> Vec<EventId> {
        let mut ids = Vec::new();
        for hit in self.entries.values() {
            if let Hit::Event(EventHit { event_id, .. }) = hit {
                ids.push(*event_id);
            }
        }
        ids
    }

    /// Collects every name from [`NameHit`]s of the given kind.
    pub(crate) fn names_of(&self, kind: NameKind) -> Vec<String> {
        let mut names = Vec::new();
        for hit in self.entries.values() {
            if let Hit::Name(NameHit {
                name,
                kind: hit_kind,
                ..
            }) = hit
                && *hit_kind == kind
            {
                names.push(name.clone());
            }
        }
        names
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

    #[test]
    fn filter_retains_only_matching_hits() {
        let a = entity_hit("a", Some(1.0));
        let b = entity_hit("b", Some(2.0));
        let sel = selection_of(vec![a.clone(), b.clone()]);
        // Filter keeps only hits from entity_hit with score > 1.0.
        // We use a discriminator that works: only "b" has score 2.0.
        let filtered = sel.filter(|h| h.relevance().score() == Some(2.0));
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn filter_can_produce_empty_selection() {
        let a = entity_hit("a", None);
        let sel = selection_of(vec![a]);
        let filtered = sel.filter(|_| false);
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn paginate_respects_offset_and_limit() {
        let a = entity_hit("a", Some(1.0));
        let b = entity_hit("b", Some(2.0));
        let c = entity_hit("c", Some(3.0));
        let sel = selection_of(vec![a, b, c]);
        let page = sel.paginate(1, 1);
        assert_eq!(page.len(), 1);
    }

    #[test]
    fn paginate_empty_selection_returns_empty() {
        let sel = Selection::new();
        let page = sel.paginate(0, 10);
        assert!(page.is_empty());
    }

    #[test]
    fn paginate_offset_past_end_returns_empty() {
        let a = entity_hit("a", None);
        let sel = selection_of(vec![a]);
        let page = sel.paginate(10, 10);
        assert!(page.is_empty());
    }
}
