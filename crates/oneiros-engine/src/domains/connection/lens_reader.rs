use std::collections::HashSet;

use rusqlite::params;

use crate::*;

pub(crate) struct ConnectionLensReader<'a> {
    db: &'a BookmarkDb,
}

impl<'a> ConnectionLensReader<'a> {
    pub(crate) fn new(db: &'a BookmarkDb) -> Self {
        Self { db }
    }

    fn neighbors_from(&self, source: &Ref) -> Result<Vec<(Ref, Timestamp)>, ReaderError> {
        let source_json =
            serde_json::to_string(source).map_err(|e| ReaderError::Internal(e.to_string()))?;
        let mut stmt = self
            .db
            .prepare(
                "SELECT to_ref, created_at
                 FROM connections
                 WHERE from_ref = ?1
                 ORDER BY created_at DESC",
            )
            .map_err(|e| ReaderError::Internal(e.to_string()))?;
        let rows = stmt
            .query_map(params![source_json], |row| {
                let to_ref: String = row.get(0)?;
                let created_at: String = row.get(1)?;
                Ok((to_ref, created_at))
            })
            .map_err(|e| ReaderError::Internal(e.to_string()))?;
        let mut out = Vec::new();
        for row in rows {
            let (to_ref_raw, created_at) = row.map_err(|e| ReaderError::Internal(e.to_string()))?;
            let to_ref: Ref = serde_json::from_str(&to_ref_raw)
                .map_err(|e| ReaderError::Internal(e.to_string()))?;
            let timestamp = Timestamp::parse_str(&created_at)
                .map_err(|e| ReaderError::Internal(e.to_string()))?;
            out.push((to_ref, timestamp));
        }
        Ok(out)
    }

    fn neighbors_to(&self, target: &Ref) -> Result<Vec<(Ref, Timestamp)>, ReaderError> {
        let target_json =
            serde_json::to_string(target).map_err(|e| ReaderError::Internal(e.to_string()))?;
        let mut stmt = self
            .db
            .prepare(
                "SELECT from_ref, created_at
                 FROM connections
                 WHERE to_ref = ?1
                 ORDER BY created_at DESC",
            )
            .map_err(|e| ReaderError::Internal(e.to_string()))?;
        let rows = stmt
            .query_map(params![target_json], |row| {
                let from_ref: String = row.get(0)?;
                let created_at: String = row.get(1)?;
                Ok((from_ref, created_at))
            })
            .map_err(|e| ReaderError::Internal(e.to_string()))?;
        let mut out = Vec::new();
        for row in rows {
            let (from_ref_raw, created_at) =
                row.map_err(|e| ReaderError::Internal(e.to_string()))?;
            let from_ref: Ref = serde_json::from_str(&from_ref_raw)
                .map_err(|e| ReaderError::Internal(e.to_string()))?;
            let timestamp = Timestamp::parse_str(&created_at)
                .map_err(|e| ReaderError::Internal(e.to_string()))?;
            out.push((from_ref, timestamp));
        }
        Ok(out)
    }

    fn step_connected_from(&self, input: &Selection) -> Result<Selection, ReaderError> {
        let mut selection = Selection::new();
        for source in input.entity_refs() {
            for (neighbor, timestamp) in self.neighbors_from(&source)? {
                selection.insert(Hit::Entity(EntityHit {
                    entity_ref: neighbor,
                    timestamp,
                    relevance: Relevance::Unknown,
                }));
            }
        }
        Ok(selection)
    }

    fn step_connected_to(&self, input: &Selection) -> Result<Selection, ReaderError> {
        let mut selection = Selection::new();
        for target in input.entity_refs() {
            for (neighbor, timestamp) in self.neighbors_to(&target)? {
                selection.insert(Hit::Entity(EntityHit {
                    entity_ref: neighbor,
                    timestamp,
                    relevance: Relevance::Unknown,
                }));
            }
        }
        Ok(selection)
    }

    fn walk(
        &self,
        input: &Selection,
        max_depth: Option<u32>,
        forward: bool,
        backward: bool,
    ) -> Result<Selection, ReaderError> {
        let seeds = input.entity_refs();
        let mut visited: HashSet<Ref> = seeds.iter().cloned().collect();
        let mut frontier: Vec<Ref> = seeds;
        let mut selection = Selection::new();
        let mut depth: u32 = 0;

        while !frontier.is_empty() {
            if let Some(limit) = max_depth
                && depth >= limit
            {
                break;
            }
            depth += 1;
            let mut next_frontier = Vec::new();
            for node in &frontier {
                let mut neighbors: Vec<(Ref, Timestamp)> = Vec::new();
                if forward {
                    neighbors.extend(self.neighbors_from(node)?);
                }
                if backward {
                    neighbors.extend(self.neighbors_to(node)?);
                }
                for (neighbor, timestamp) in neighbors {
                    if visited.insert(neighbor.clone()) {
                        selection.insert(Hit::Entity(EntityHit {
                            entity_ref: neighbor.clone(),
                            timestamp,
                            relevance: Relevance::Unknown,
                        }));
                        next_frontier.push(neighbor);
                    }
                }
            }
            frontier = next_frontier;
        }
        Ok(selection)
    }
}

impl Reader for ConnectionLensReader<'_> {
    fn read(&self, _read: &Read) -> Option<Result<Selection, ReaderError>> {
        None
    }

    fn step(&self, kind: &StepKind, input: &Selection) -> Option<Result<Selection, ReaderError>> {
        match kind {
            StepKind::ConnectedFrom => Some(self.step_connected_from(input)),
            StepKind::ConnectedTo => Some(self.step_connected_to(input)),
            StepKind::Descendants => Some(self.walk(input, None, true, false)),
            StepKind::Ancestors => Some(self.walk(input, None, false, true)),
            StepKind::Within(n) => Some(self.walk(input, Some(*n), true, true)),
            StepKind::Component => Some(self.walk(input, None, true, true)),
            StepKind::EventsFor
            | StepKind::RefsFrom
            | StepKind::SearchByAgent
            | StepKind::SearchByTexture
            | StepKind::SearchByLevel
            | StepKind::SearchByKind => None,
        }
    }
}
