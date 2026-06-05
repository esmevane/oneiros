use rusqlite::params;

use crate::*;

type RemoteRow = (
    String, // id
    String, // name
    String, // address
    String, // ticket_token
    String, // ticket_target
    String, // project_name
    String, // created_at
);

const SELECT_COLUMNS: &str =
    "id, name, address, ticket_token, ticket_target, project_name, created_at";

fn remote_from_row(row: RemoteRow) -> Result<Remote, EventError> {
    let (id, name, address, ticket_token, ticket_target, project_name, created_at) = row;

    let target_ref: RefToken = ticket_target
        .parse()
        .map_err(|e: RefError| EventError::Import(e.to_string()))?;
    let link = Link::new(target_ref.into_inner(), Token::from(ticket_token));

    Ok(Remote {
        id: id.parse()?,
        name: RemoteName::new(name),
        address: address
            .parse()
            .map_err(|e: PeerAddressError| EventError::Import(e.to_string()))?,
        ticket: link,
        project: ProjectName::new(project_name),
        created_at: Timestamp::parse_str(created_at)?,
    })
}

fn read_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<RemoteRow> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
        row.get(6)?,
    ))
}

pub(crate) struct RemoteRepo<'a> {
    scope: &'a Scope<AtHost>,
}

impl<'a> RemoteRepo<'a> {
    pub(crate) fn new(scope: &'a Scope<AtHost>) -> Self {
        Self { scope }
    }

    pub(crate) async fn fetch(&self, id: &RemoteId) -> Result<Option<Remote>, EventError> {
        self.scope.config().fetch.eventual(|| self.get(id)).await
    }

    pub(crate) async fn get(&self, id: &RemoteId) -> Result<Option<Remote>, EventError> {
        let db = HostDb::open(self.scope).await?;
        let sql = format!("SELECT {SELECT_COLUMNS} FROM remotes WHERE id = ?1");
        let mut stmt = db.prepare(&sql)?;
        let raw: Result<RemoteRow, _> = stmt.query_row(params![id.to_string()], read_row);
        match raw {
            Ok(row) => Ok(Some(remote_from_row(row)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub(crate) async fn get_by_name(
        &self,
        name: &RemoteName,
    ) -> Result<Option<Remote>, EventError> {
        let db = HostDb::open(self.scope).await?;
        let sql = format!("SELECT {SELECT_COLUMNS} FROM remotes WHERE name = ?1");
        let mut stmt = db.prepare(&sql)?;
        let raw: Result<RemoteRow, _> = stmt.query_row(params![name.to_string()], read_row);
        match raw {
            Ok(row) => Ok(Some(remote_from_row(row)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub(crate) async fn list(&self, filters: &SearchFilters) -> Result<Listed<Remote>, EventError> {
        let db = HostDb::open(self.scope).await?;
        let total = {
            let mut stmt = db.prepare("SELECT COUNT(*) FROM remotes")?;
            stmt.query_row([], |row| row.get::<_, usize>(0))?
        };
        let sql = format!(
            "SELECT {SELECT_COLUMNS} FROM remotes ORDER BY created_at DESC LIMIT ?1 OFFSET ?2"
        );
        let mut stmt = db.prepare(&sql)?;
        let raw: Vec<RemoteRow> = stmt
            .query_map(rusqlite::params![filters.limit, filters.offset], read_row)?
            .collect::<Result<Vec<_>, _>>()?;
        let mut remotes = vec![];
        for row in raw {
            remotes.push(remote_from_row(row)?);
        }
        Ok(Listed::new(remotes, total))
    }
}
