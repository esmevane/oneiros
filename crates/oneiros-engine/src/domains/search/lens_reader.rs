use rusqlite::params;

use crate::*;

pub(crate) struct SearchIndexReader<'a> {
    db: &'a BookmarkDb,
}

impl<'a> SearchIndexReader<'a> {
    pub(crate) fn new(db: &'a BookmarkDb) -> Self {
        Self { db }
    }

    fn read_facet(&self, facet: FacetName, value: &str) -> Result<Selection, ReaderError> {
        let resolved_value = match facet {
            FacetName::Agent => self.resolve_agent_name(value)?,
            _ => value.to_string(),
        };
        let column = facet.column();
        let sql = format!(
            "select resource_ref, created_at from search_index where {column} = ?1 order by created_at desc"
        );
        let mut stmt = self
            .db
            .prepare(&sql)
            .map_err(|e| ReaderError::Internal(e.to_string()))?;
        let mut selection = Selection::new();
        let rows = stmt
            .query_map(params![resolved_value], |row| {
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
}

impl Reader for SearchIndexReader<'_> {
    fn read(&self, read: &Read) -> Option<Result<Selection, ReaderError>> {
        match read {
            Read::SearchFacet { facet, value } => Some(self.read_facet(*facet, value)),
            Read::SearchText(query) => Some(self.read_text(query)),
        }
    }
}
