use crate::*;

pub struct BookmarkRepo<'a> {
    scope: &'a Scope<AtHost>,
}

impl<'a> BookmarkRepo<'a> {
    pub fn new(scope: &'a Scope<AtHost>) -> Self {
        Self { scope }
    }

    pub async fn list(
        &self,
        brain: &BrainName,
        filters: &SearchFilters,
    ) -> Result<Listed<Bookmark>, BookmarkError> {
        let db = self.scope.host_db().await?;

        let count_sql = "SELECT COUNT(*) FROM bookmarks WHERE brain = ?1";
        let total = {
            let mut stmt = db.prepare(count_sql)?;
            stmt.query_row(rusqlite::params![brain.to_string()], |row| {
                row.get::<_, usize>(0)
            })?
        };

        let mut stmt = db.prepare(
            "SELECT id, brain, name, created_at FROM bookmarks
             WHERE brain = ?1
             ORDER BY created_at DESC
             LIMIT ?2 OFFSET ?3",
        )?;

        let raw: Vec<(String, String, String, String)> = stmt
            .query_map(
                rusqlite::params![brain.to_string(), filters.limit, filters.offset],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )?
            .collect::<Result<_, _>>()?;

        let mut bookmarks = vec![];

        for (id, brain, name, created_at) in raw {
            bookmarks.push(Bookmark {
                id: id.parse()?,
                brain: BrainName::new(brain),
                name: BookmarkName::new(name),
                created_at: Timestamp::parse_str(created_at)?,
            });
        }

        Ok(Listed::new(bookmarks, total))
    }
}
