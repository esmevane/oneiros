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

    /// Execute a search with optional filters. Returns hits plus facet
    /// aggregations scoped to the filtered result set.
    ///
    /// `agent_id` is resolved by the caller (service layer) from an
    /// optional `AgentName` on the public query shape.
    pub async fn search(
        &self,
        query: &SearchQuery,
        agent_id: Option<&AgentId>,
    ) -> Result<SearchResults, EventError> {
        let db = self.context.db()?;

        let has_query = query.query.is_some();
        let (where_clause, bindings) = build_filters(query, agent_id);
        let params: Vec<&dyn ToSql> = bindings.iter().map(|b| b.as_ref() as &dyn ToSql).collect();

        let total = {
            let sql = format!("select count(*) from search_index{where_clause}");
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
                "select resource_ref, kind, content, agent_id, texture, level, sensation, persona, created_at
                 from search_index{where_clause}
                 {order}
                 limit ?{limit_idx} offset ?{offset_idx}",
                limit_idx = bindings.len() + 1,
                offset_idx = bindings.len() + 2,
            );
            let limit: Box<dyn ToSql> = Box::new(query.filters.limit.0 as i64);
            let offset: Box<dyn ToSql> = Box::new(query.filters.offset.0 as i64);
            let mut paged: Vec<&dyn ToSql> = params.clone();
            paged.push(limit.as_ref());
            paged.push(offset.as_ref());
            let mut statement = db.prepare(&sql)?;
            statement
                .query_map(params_from_iter(&paged), map_hit)?
                .collect::<Result<Vec<_>, _>>()?
        };

        let facets = collect_facets(&db, &where_clause, &params)?;

        Ok(SearchResults {
            query: QueryText::new(query.query.clone().unwrap_or_default()),
            total,
            hits,
            facets,
        })
    }
}

/// Build the WHERE clause + bindings shared by the hit query and facet
/// aggregations. The FTS5 `match` condition is only included when a text
/// query is present; otherwise we fall back to column-only filtering.
fn build_filters(query: &SearchQuery, agent_id: Option<&AgentId>) -> (String, Vec<Box<dyn ToSql>>) {
    let mut conditions: Vec<String> = Vec::new();
    let mut bindings: Vec<Box<dyn ToSql>> = Vec::new();

    if let Some(q) = &query.query {
        bindings.push(Box::new(q.clone()));
        conditions.push(format!("search_index match ?{}", bindings.len()));
    }
    if let Some(kind) = query.kind {
        bindings.push(Box::new(kind.as_str().to_string()));
        conditions.push(format!("kind = ?{}", bindings.len()));
    }
    if let Some(agent_id) = agent_id {
        bindings.push(Box::new(agent_id.to_string()));
        conditions.push(format!("agent_id = ?{}", bindings.len()));
    }
    if let Some(texture) = &query.texture {
        bindings.push(Box::new(texture.to_string()));
        conditions.push(format!("texture = ?{}", bindings.len()));
    }
    if let Some(level) = &query.level {
        bindings.push(Box::new(level.to_string()));
        conditions.push(format!("level = ?{}", bindings.len()));
    }
    if let Some(sensation) = &query.sensation {
        bindings.push(Box::new(sensation.to_string()));
        conditions.push(format!("sensation = ?{}", bindings.len()));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!(" where {}", conditions.join(" and "))
    };
    (where_clause, bindings)
}

/// Run GROUP BY over each facet column, scoped to the same WHERE clause as
/// the hit query. Empty bucket values are excluded so we only surface
/// dimensions that actually apply to the result set.
fn collect_facets(
    db: &rusqlite::Connection,
    where_clause: &str,
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
        let joiner = if where_clause.is_empty() {
            "where"
        } else {
            "and"
        };
        let sql = format!(
            "select {column}, count(*) as n
             from search_index{where_clause} {joiner} {column} != ''
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

fn map_hit(row: &rusqlite::Row) -> rusqlite::Result<Expression> {
    let ref_json: String = row.get(0)?;
    let resource_ref: Ref = serde_json::from_str(&ref_json).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
    })?;

    Ok(Expression::builder()
        .resource_ref(resource_ref)
        .kind(row.get::<_, String>(1)?)
        .content(row.get::<_, String>(2)?)
        .maybe_agent(parse_optional(row.get::<_, String>(3)?))
        .maybe_texture(parse_optional(row.get::<_, String>(4)?))
        .maybe_level(parse_optional(row.get::<_, String>(5)?))
        .maybe_sensation(parse_optional(row.get::<_, String>(6)?))
        .maybe_persona(parse_optional(row.get::<_, String>(7)?))
        .maybe_created_at(parse_timestamp(row.get::<_, String>(8)?))
        .build())
}

fn parse_optional<T: core::str::FromStr>(raw: String) -> Option<T> {
    if raw.is_empty() {
        None
    } else {
        raw.parse().ok()
    }
}

fn parse_timestamp(raw: String) -> Option<Timestamp> {
    if raw.is_empty() {
        None
    } else {
        Timestamp::parse_str(&raw).ok()
    }
}
