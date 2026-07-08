use rusqlite::params;

use crate::*;

pub(crate) struct SearchIndexReader<'a> {
    db: &'a DbHandle<'a>,
}

impl<'a> SearchIndexReader<'a> {
    pub(crate) fn new(db: &'a DbHandle<'a>) -> Self {
        Self { db }
    }

    fn read_text(&self, query: &str) -> Result<Selection, ReaderError> {
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
            Err(DbError::Rusqlite(rusqlite::Error::QueryReturnedNoRows)) => Ok(name.to_string()),
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
