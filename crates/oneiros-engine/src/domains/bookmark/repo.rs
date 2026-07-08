use crate::*;

pub(crate) struct BookmarkRepo<'a> {
    scope: &'a Scope<AtHost>,
}

impl<'a> BookmarkRepo<'a> {
    pub(crate) fn new(scope: &'a Scope<AtHost>) -> Self {
        Self { scope }
    }

    pub(crate) async fn list(
        &self,
        project: &ProjectName,
        filters: &SearchFilters,
    ) -> Result<Listed<Bookmark>, BookmarkError> {
        let db = self.scope.host_db().await?;

        let count_sql = "SELECT COUNT(*) FROM bookmarks WHERE project = ?1";
        let total = db.query_row(count_sql, rusqlite::params![project.to_string()], |row| {
            row.get::<_, usize>(0)
        })?;

        let raw: Vec<(String, String, String, String)> = db.query_map(
            "SELECT id, project, name, created_at FROM bookmarks
             WHERE project = ?1
             ORDER BY created_at DESC
             LIMIT ?2 OFFSET ?3",
            rusqlite::params![project.to_string(), filters.limit, filters.offset],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )?;

        let mut bookmarks = vec![];

        for (id, project, name, created_at) in raw {
            bookmarks.push(Bookmark {
                id: id.parse()?,
                project: ProjectName::new(project),
                name: BookmarkName::new(name),
                created_at: Timestamp::parse_str(created_at)?,
            });
        }

        Ok(Listed::new(bookmarks, total))
    }
}
