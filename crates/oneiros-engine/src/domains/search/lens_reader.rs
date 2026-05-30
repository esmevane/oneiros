use rusqlite::params;

use crate::*;

pub(crate) struct SearchIndexReader<'a> {
    db: &'a BookmarkDb,
}

impl<'a> SearchIndexReader<'a> {
    pub(crate) fn new(db: &'a BookmarkDb) -> Self {
        Self { db }
    }

    fn read_text(&self, query: &str) -> Result<Selection, ReaderError> {
        let sql = "select resource_ref, created_at, rank from search_index where search_index match ?1 order by rank";
        let mut stmt = self
            .db
            .prepare(sql)
            .map_err(|e| ReaderError::Internal(e.to_string()))?;
        let mut selection = Selection::new();
        let rows = stmt
            .query_map(params![query], |row| {
                let ref_json: String = row.get(0)?;
                let created_at: String = row.get(1)?;
                let rank: f64 = row.get(2)?;
                Ok((ref_json, created_at, rank))
            })
            .map_err(|e| ReaderError::Internal(e.to_string()))?;

        for row in rows {
            let (ref_json, created_at, rank) =
                row.map_err(|e| ReaderError::Internal(e.to_string()))?;
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

    fn step_by_column(
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
        let mut stmt = self
            .db
            .prepare(&sql)
            .map_err(|e| ReaderError::Internal(e.to_string()))?;

        let bind: Vec<&dyn rusqlite::ToSql> =
            resolved.iter().map(|s| s as &dyn rusqlite::ToSql).collect();

        let mut selection = Selection::new();
        let rows = stmt
            .query_map(bind.as_slice(), |row| {
                let ref_json: String = row.get(0)?;
                let created_at: String = row.get(1)?;
                Ok((ref_json, created_at))
            })
            .map_err(|e| ReaderError::Internal(e.to_string()))?;

        for row in rows {
            let (ref_json, created_at) = row.map_err(|e| ReaderError::Internal(e.to_string()))?;
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
        let result: Result<String, _> = self.db.query_row(sql, params![name], |row| row.get(0));
        match result {
            Ok(id) => Ok(id),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(name.to_string()),
            Err(e) => Err(ReaderError::Internal(e.to_string())),
        }
    }
}

impl Reader for SearchIndexReader<'_> {
    fn read(&self, read: &Read) -> Option<Result<Selection, ReaderError>> {
        match read {
            Read::SearchText(query) => Some(self.read_text(query)),
            _ => None,
        }
    }

    fn step(&self, kind: &StepKind, input: &Selection) -> Option<Result<Selection, ReaderError>> {
        let (column, name_kind, resolve_agent) = match kind {
            StepKind::SearchByAgent => ("agent_id", NameKind::Agent, true),
            StepKind::SearchByTexture => ("texture", NameKind::Texture, false),
            StepKind::SearchByLevel => ("level", NameKind::Level, false),
            StepKind::SearchByKind => ("kind", NameKind::Kind, false),
            _ => return None,
        };
        let names = input.names_of(name_kind);
        Some(self.step_by_column(column, names, resolve_agent))
    }
}
