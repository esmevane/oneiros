use rusqlite::params;
use std::future::Future;
use std::pin::Pin;

use crate::*;

pub(crate) struct SearchIndexReader<'a> {
    db: &'a DbHandle,
}

impl<'a> SearchIndexReader<'a> {
    pub(crate) fn new(db: &'a DbHandle) -> Self {
        Self { db }
    }

    pub(crate) fn read_text(&self, query: &str) -> Result<Selection, ReaderError> {
        let sql = "select resource_ref, created_at, rank from search_index where search_index match ?1 order by rank";
        let tuples = self
            .db
            .query_map(sql, params![query], |row| {
                let ref_json: String = row.get(0)?;
                let created_at: String = row.get(1)?;
                let rank: f64 = row.get(2)?;
                Ok((ref_json, created_at, rank))
            })
            .map_err(|e| ReaderError::Internal(e.to_string()))?;

        let mut selection = Selection::new();
        for (ref_json, created_at, rank) in tuples {
            let entity_ref: Ref = serde_json::from_str(&ref_json)
                .map_err(|e| ReaderError::Internal(e.to_string()))?;
            let timestamp = if created_at.is_empty() {
                Timestamp::now()
            } else {
                Timestamp::parse_str(&created_at)
                    .map_err(|e| ReaderError::Internal(e.to_string()))?
            };
            let score = -rank;
            selection.insert(Hit::Entity(EntityHit {
                entity_ref,
                timestamp,
                relevance: Relevance::Known { score },
            }));
        }
        Ok(selection)
    }

    pub(crate) fn step_by_column(
        &self,
        column: &str,
        names: Vec<String>,
        resolve_agent: bool,
    ) -> Result<Selection, ReaderError> {
        if names.is_empty() {
            return Ok(Selection::new());
        }

        let resolved: Vec<String> = if resolve_agent {
            let mut out = Vec::with_capacity(names.len());
            for name in &names {
                out.push(self.resolve_agent_name(name)?);
            }
            out
        } else {
            names
        };

        let placeholders = std::iter::repeat_n("?", resolved.len())
            .collect::<Vec<_>>()
            .join(", ");
        let sql = format!(
            "select resource_ref, created_at from search_index where {column} in ({placeholders}) order by created_at desc"
        );

        let bind: Vec<&dyn rusqlite::ToSql> =
            resolved.iter().map(|s| s as &dyn rusqlite::ToSql).collect();

        let tuples = self
            .db
            .query_map(&sql, bind.as_slice(), |row: &DbRow<'_>| {
                let ref_json: String = row.get(0)?;
                let created_at: String = row.get(1)?;
                Ok((ref_json, created_at))
            })
            .map_err(|e| ReaderError::Internal(e.to_string()))?;

        let mut selection = Selection::new();
        for (ref_json, created_at) in tuples {
            let entity_ref: Ref = serde_json::from_str(&ref_json)
                .map_err(|e| ReaderError::Internal(e.to_string()))?;
            let timestamp = if created_at.is_empty() {
                Timestamp::now()
            } else {
                Timestamp::parse_str(&created_at)
                    .map_err(|e| ReaderError::Internal(e.to_string()))?
            };
            selection.insert(Hit::Entity(EntityHit {
                entity_ref,
                timestamp,
                relevance: Relevance::Unknown,
            }));
        }
        Ok(selection)
    }

    fn resolve_agent_name(&self, name: &str) -> Result<String, ReaderError> {
        let sql = "select id from agents where name = ?1";
        let result = self
            .db
            .query_row(sql, params![name], |row| row.get::<_, String>(0));
        match result {
            Ok(id) => Ok(id),
            Err(DbError::NotFound) => Ok(name.to_string()),
            Err(e) => Err(ReaderError::Internal(e.to_string())),
        }
    }
}

impl LensReader for SearchIndexReader<'_> {
    fn read<'a>(
        &'a self,
        read: &'a LensRead,
    ) -> Pin<Box<dyn Future<Output = Option<Result<Selection, ReaderError>>> + Send + 'a>> {
        match read {
            LensRead::SearchText(query) => Box::pin(async move { Some(self.read_text(query)) }),
            LensRead::ChronicleBetween { .. } => Box::pin(async { None }),
        }
    }

    fn step(
        &self,
        kind: &LensStepKind,
        input: &Selection,
    ) -> Option<Result<Selection, ReaderError>> {
        match kind {
            LensStepKind::SearchByAgent => {
                Some(self.step_by_column("agent_id", input.names_of(NameKind::Agent), true))
            }
            LensStepKind::SearchByTexture => {
                Some(self.step_by_column("texture", input.names_of(NameKind::Texture), false))
            }
            LensStepKind::SearchByLevel => {
                Some(self.step_by_column("level", input.names_of(NameKind::Level), false))
            }
            LensStepKind::SearchByKind => {
                Some(self.step_by_column("kind", input.names_of(NameKind::Kind), false))
            }
            LensStepKind::EventsFor
            | LensStepKind::RefsFrom
            | LensStepKind::ConnectedFrom
            | LensStepKind::ConnectedTo
            | LensStepKind::Descendants
            | LensStepKind::Ancestors
            | LensStepKind::Within(_)
            | LensStepKind::Component => None,
        }
    }
}
