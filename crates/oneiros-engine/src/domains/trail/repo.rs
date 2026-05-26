use rusqlite::params;

use crate::*;

/// Trail read model — async queries over the projection.
pub(crate) struct TrailRepo<'a> {
    scope: &'a Scope<AtBookmark>,
}

impl<'a> TrailRepo<'a> {
    pub(crate) fn new(scope: &'a Scope<AtBookmark>) -> Self {
        Self { scope }
    }

    /// Events that touched the given entity, oldest first.
    pub(crate) async fn events_for(
        &self,
        entity_ref: &RefToken,
    ) -> Result<Vec<TrailEntry>, EventError> {
        let db = BookmarkDb::open(self.scope).await?;
        let mut stmt = db.prepare(
            "SELECT event_id, ref, event_type, created_at
             FROM trail
             WHERE ref = ?1
             ORDER BY created_at ASC",
        )?;

        let rows: Vec<TrailRow> = stmt
            .query_map(params![entity_ref.to_string()], read_row)?
            .collect::<Result<Vec<_>, _>>()?;

        rows.into_iter().map(entry_from_row).collect()
    }

    /// Entity refs the given event emitted, in insertion order.
    pub(crate) async fn refs_from(&self, event_id: EventId) -> Result<Vec<RefToken>, EventError> {
        let db = BookmarkDb::open(self.scope).await?;
        let mut stmt = db.prepare(
            "SELECT ref
             FROM trail
             WHERE event_id = ?1
             ORDER BY ref ASC",
        )?;

        let rows = stmt
            .query_map(params![event_id.to_string()], |row| row.get::<_, String>(0))?
            .collect::<Result<Vec<_>, _>>()?;

        let mut refs = Vec::with_capacity(rows.len());
        for raw in rows {
            refs.push(raw.parse()?);
        }
        Ok(refs)
    }
}

type TrailRow = (String, String, String, String);

fn read_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<TrailRow> {
    Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
}

fn entry_from_row(
    (event_id, entity_ref, event_type, created_at): TrailRow,
) -> Result<TrailEntry, EventError> {
    Ok(TrailEntry::builder()
        .event_id(event_id.parse()?)
        .entity_ref(entity_ref.parse()?)
        .event_type(event_type)
        .created_at(Timestamp::parse_str(&created_at)?)
        .build())
}
