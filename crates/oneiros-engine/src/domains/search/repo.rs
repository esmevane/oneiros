use rusqlite::{ToSql, params_from_iter};

use crate::*;

/// Search repo — async read queries over the FTS5 search index.
///
/// Returns faceted results: hits (with typed per-kind metadata) plus
/// aggregations grouped by kind, agent, texture, level, sensation, and
/// persona. Facet groups with no buckets are omitted.
///
/// With a text query, hits are ranked by FTS5 relevance. Without one, the
/// repo browses by filters alone, ordered by `created_at` descending — the
/// shape list endpoints consume.
pub struct SearchRepo<'a> {
    context: &'a ProjectContext,
}

impl<'a> SearchRepo<'a> {
    pub fn new(context: &'a ProjectContext) -> Self {
        Self { context }
    }

    /// Execute a search with optional filters. Returns ranked refs plus
    /// facet aggregations scoped to the filtered result set. Hydration to
    /// typed [`Hit`]s is the service layer's job — the repo only knows
    /// what's in the index.
    ///
    /// `agent_id` is resolved by the caller (service layer) from an
    /// optional `AgentName` on the public query shape.
    pub(crate) async fn search(
        &self,
        query: &SearchQueryV1,
        agent_id: Option<&AgentId>,
    ) -> Result<SearchHits, EventError> {
        let db = self.context.db()?;

        let has_query = query.query.is_some();
        let where_clause = query.where_clause(agent_id);
        let params = where_clause.params();

        let total = {
            let sql = format!("select count(*) from search_index{}", where_clause.sql);
            db.prepare(&sql)?
                .query_row(params_from_iter(&params), |row| row.get::<_, usize>(0))?
        };

        let hits = {
            let order = if has_query {
                "order by rank"
            } else {
                "order by created_at desc"
            };
            let sql = format!(
                "select resource_ref from search_index{where_sql}
                 {order}
                 limit ?{limit_idx} offset ?{offset_idx}",
                where_sql = where_clause.sql,
                limit_idx = where_clause.bindings.len() + 1,
                offset_idx = where_clause.bindings.len() + 2,
            );
            let limit: Box<dyn ToSql> = Box::new(query.filters.limit.0 as i64);
            let offset: Box<dyn ToSql> = Box::new(query.filters.offset.0 as i64);
            let mut paged: Vec<&dyn ToSql> = params.clone();
            paged.push(limit.as_ref());
            paged.push(offset.as_ref());
            let mut statement = db.prepare(&sql)?;
            statement
                .query_map(params_from_iter(&paged), RankedHit::from_row)?
                .collect::<Result<Vec<_>, _>>()?
        };

        let facets = if query.with_facets {
            self.collect_facets(&db, &where_clause.sql, &params)?
        } else {
            Facets::default()
        };

        Ok(SearchHits {
            total,
            hits,
            facets,
        })
    }

    /// Run GROUP BY over each facet column, scoped to the same WHERE clause
    /// as the hit query. Empty bucket values are excluded so we only surface
    /// dimensions that actually apply to the result set.
    fn collect_facets(
        &self,
        db: &rusqlite::Connection,
        where_sql: &str,
        params: &[&dyn ToSql],
    ) -> Result<Facets, EventError> {
        let mut groups = Vec::new();
        for facet in [
            FacetName::Kind,
            FacetName::Agent,
            FacetName::Texture,
            FacetName::Level,
            FacetName::Sensation,
            FacetName::Persona,
        ] {
            let column = facet.column();
            let joiner = if where_sql.is_empty() { "where" } else { "and" };
            let sql = format!(
                "select {column}, count(*) as n
                 from search_index{where_sql} {joiner} {column} != ''
                 group by {column}
                 order by n desc, {column} asc"
            );
            let mut stmt = db.prepare(&sql)?;
            let rows = stmt
                .query_map(params_from_iter(params), |row| {
                    Ok(FacetBucket {
                        value: row.get::<_, String>(0)?,
                        count: row.get::<_, usize>(1)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;

            if !rows.is_empty() {
                groups.push(FacetGroup {
                    facet,
                    buckets: rows,
                });
            }
        }
        Ok(Facets(groups))
    }
}

/// SQL fragment built from a `SearchQuery`: the `where ...` clause and its
/// owned parameter bindings. Kept together so the same scope drives both the
/// hit query and facet aggregations.
pub(crate) struct WhereClause {
    pub sql: String,
    pub bindings: Vec<Box<dyn ToSql>>,
}

impl WhereClause {
    pub fn params(&self) -> Vec<&dyn ToSql> {
        self.bindings
            .iter()
            .map(|b| b.as_ref() as &dyn ToSql)
            .collect()
    }
}

impl SearchQueryV1 {
    /// Translate the query's filters into a WHERE clause + parameter
    /// bindings. The FTS5 `match` condition is only included when a text
    /// query is present; otherwise we fall back to column-only filtering.
    pub(crate) fn where_clause(&self, agent_id: Option<&AgentId>) -> WhereClause {
        let mut conditions: Vec<String> = Vec::new();
        let mut bindings: Vec<Box<dyn ToSql>> = Vec::new();

        let mut push = |condition: String, binding: Box<dyn ToSql>| {
            bindings.push(binding);
            conditions.push(condition.replace("?N", &format!("?{}", bindings.len())));
        };

        if let Some(q) = &self.query {
            push("search_index match ?N".into(), Box::new(q.clone()));
        }
        if let Some(kind) = self.kind {
            push("kind = ?N".into(), Box::new(kind.as_str().to_string()));
        }
        if let Some(agent_id) = agent_id {
            push("agent_id = ?N".into(), Box::new(agent_id.to_string()));
        }
        if let Some(texture) = &self.texture {
            push("texture = ?N".into(), Box::new(texture.to_string()));
        }
        if let Some(level) = &self.level {
            push("level = ?N".into(), Box::new(level.to_string()));
        }
        if let Some(sensation) = &self.sensation {
            push("sensation = ?N".into(), Box::new(sensation.to_string()));
        }

        let sql = if conditions.is_empty() {
            String::new()
        } else {
            format!(" where {}", conditions.join(" and "))
        };
        WhereClause { sql, bindings }
    }
}
