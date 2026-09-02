use rusqlite::params;
use std::future::Future;
use std::pin::Pin;

use crate::*;

pub(crate) struct TrailLensReader<'a> {
    db: &'a DbHandle,
}

impl<'a> TrailLensReader<'a> {
    pub(crate) fn new(db: &'a DbHandle) -> Self {
        Self { db }
    }

    fn events_for_entity(
        &self,
        entity_ref: &Ref,
    ) -> Result<Vec<(EventId, Timestamp)>, ReaderError> {
        let ref_token = RefToken::new(entity_ref.clone());
        let rows = self
            .db
            .query_map(
                "SELECT event_id, created_at
                 FROM trail
                 WHERE ref = ?1
                 ORDER BY created_at DESC",
                params![ref_token.to_string()],
                |row| {
                    let event_id: String = row.get(0)?;
                    let created_at: String = row.get(1)?;
                    Ok((event_id, created_at))
                },
            )
            .map_err(|e| ReaderError::Internal(e.to_string()))?;
        let mut out = Vec::new();
        for (event_id_raw, created_at) in rows {
            let event_id: EventId = event_id_raw
                .parse()
                .map_err(|e: IdParseError| ReaderError::Internal(e.to_string()))?;
            let timestamp = Timestamp::parse_str(&created_at)
                .map_err(|e| ReaderError::Internal(e.to_string()))?;
            out.push((event_id, timestamp));
        }
        Ok(out)
    }

    fn refs_emitted_by(&self, event_id: &EventId) -> Result<Vec<(Ref, Timestamp)>, ReaderError> {
        let rows = self
            .db
            .query_map(
                "SELECT ref, created_at
                 FROM trail
                 WHERE event_id = ?1
                 ORDER BY created_at DESC",
                params![event_id.to_string()],
                |row| {
                    let ref_token: String = row.get(0)?;
                    let created_at: String = row.get(1)?;
                    Ok((ref_token, created_at))
                },
            )
            .map_err(|e| ReaderError::Internal(e.to_string()))?;
        let mut out = Vec::new();
        for (ref_token_raw, created_at) in rows {
            let ref_token: RefToken = ref_token_raw
                .parse()
                .map_err(|e: RefError| ReaderError::Internal(e.to_string()))?;
            let timestamp = Timestamp::parse_str(&created_at)
                .map_err(|e| ReaderError::Internal(e.to_string()))?;
            out.push((ref_token.into(), timestamp));
        }
        Ok(out)
    }

    pub(crate) fn step_events_for(&self, input: &Selection) -> Result<Selection, ReaderError> {
        let mut selection = Selection::new();
        for entity_ref in input.entity_refs() {
            for (event_id, timestamp) in self.events_for_entity(&entity_ref)? {
                selection.insert(Hit::Event(EventHit {
                    event_id,
                    timestamp,
                    relevance: Relevance::Unknown,
                }));
            }
        }
        Ok(selection)
    }

    pub(crate) fn step_refs_from(&self, input: &Selection) -> Result<Selection, ReaderError> {
        let mut selection = Selection::new();
        for event_id in input.event_ids() {
            for (entity_ref, timestamp) in self.refs_emitted_by(&event_id)? {
                selection.insert(Hit::Entity(EntityHit {
                    entity_ref,
                    timestamp,
                    relevance: Relevance::Unknown,
                }));
            }
        }
        Ok(selection)
    }
}

impl LensReader for TrailLensReader<'_> {
    fn read<'a>(
        &'a self,
        _read: &'a LensRead,
    ) -> Pin<Box<dyn Future<Output = Option<Result<Selection, ReaderError>>> + Send + 'a>> {
        Box::pin(async { None })
    }

    fn step(
        &self,
        kind: &LensStepKind,
        input: &Selection,
    ) -> Option<Result<Selection, ReaderError>> {
        match kind {
            LensStepKind::EventsFor => Some(self.step_events_for(input)),
            LensStepKind::RefsFrom => Some(self.step_refs_from(input)),
            _ => None,
        }
    }
}
