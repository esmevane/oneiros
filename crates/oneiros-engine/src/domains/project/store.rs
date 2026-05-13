use rusqlite::params;

use crate::*;

fn is_missing_table(e: &rusqlite::Error) -> bool {
    matches!(
        e,
        rusqlite::Error::SqliteFailure(_, Some(msg))
            if msg.starts_with("no such table")
    )
}

pub(crate) struct ProjectStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> ProjectStore<'a> {
    pub(crate) fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Project(ProjectEvents::ProjectCreated(creation))) = &event.data
        {
            let project = creation.current()?.project;
            self.write_project(&project)?;
        }
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM projects", [])?;
        Ok(())
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "create table if not exists projects (
                id text primary key,
                name text not null unique,
                created_at text not null default ''
            )",
        )?;
        Ok(())
    }

    /// List all project names known to the host DB. Returns an empty
    /// list if the projection has not been migrated yet (cold start).
    pub(crate) fn list(&self) -> Result<Vec<ProjectName>, rusqlite::Error> {
        let mut stmt = match self.conn.prepare("select name from projects") {
            Ok(stmt) => stmt,
            Err(e) if is_missing_table(&e) => return Ok(Vec::new()),
            Err(e) => return Err(e),
        };
        let rows = stmt
            .query_map([], |row| row.get::<_, String>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows.into_iter().map(ProjectName::from).collect())
    }

    fn write_project(&self, project: &Project) -> Result<(), EventError> {
        self.conn.execute(
            "insert or replace into projects (id, name, created_at) values (?1, ?2, ?3)",
            params![
                project.id.to_string(),
                project.name.to_string(),
                project.created_at.to_string()
            ],
        )?;
        Ok(())
    }
}
