use rusqlite::params;

use crate::*;

/// Peer read model — async queries against the host context.
pub(crate) struct PeerRepo<'a> {
    scope: &'a Scope<AtHost>,
}

impl<'a> PeerRepo<'a> {
    pub(crate) fn new(scope: &'a Scope<AtHost>) -> Self {
        Self { scope }
    }

    /// Eventually-consistent variant of [`get`]. Polls until the
    /// peer appears or the configured patience window expires.
    ///
    /// [`get`]: PeerRepo::get
    pub(crate) async fn fetch(&self, id: PeerId) -> Result<Option<Peer>, EventError> {
        self.scope.config().fetch.eventual(|| self.get(id)).await
    }

    pub(crate) async fn get(&self, id: PeerId) -> Result<Option<Peer>, EventError> {
        let db = HostDb::open(self.scope).await?;
        let mut statement = db.prepare(
            "select id, key, address, name, kind, ticket_token, ticket_target, project, created_at from peers where id = ?1",
        )?;

        let raw = statement.query_row(params![id.to_string()], read_row);

        match raw {
            Ok(row) => Ok(Some(peer_from_row(row)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub(crate) async fn get_by_name(&self, name: &PeerName) -> Result<Option<Peer>, EventError> {
        let db = HostDb::open(self.scope).await?;
        let mut statement = db.prepare(
            "select id, key, address, name, kind, ticket_token, ticket_target, project, created_at from peers where name = ?1",
        )?;

        let raw = statement.query_row(params![name.to_string()], read_row);

        match raw {
            Ok(row) => Ok(Some(peer_from_row(row)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub(crate) async fn list(&self, filters: &SearchFilters) -> Result<Listed<Peer>, EventError> {
        let db = HostDb::open(self.scope).await?;

        let total = {
            let mut stmt = db.prepare("SELECT COUNT(*) FROM peers")?;
            stmt.query_row([], |row| row.get::<_, usize>(0))?
        };

        let select_sql = "SELECT id, key, address, name, kind, ticket_token, ticket_target, project, created_at \
                          FROM peers ORDER BY created_at DESC LIMIT ?1 OFFSET ?2";
        let mut statement = db.prepare(select_sql)?;

        let raw: Vec<PeerRow> = statement
            .query_map(rusqlite::params![filters.limit, filters.offset], read_row)?
            .collect::<Result<Vec<_>, _>>()?;

        let mut peers = vec![];

        for row in raw {
            peers.push(peer_from_row(row)?);
        }

        Ok(Listed::new(peers, total))
    }
}

type PeerRow = (
    String, // id
    String, // key
    String, // address
    String, // name
    String, // kind
    String, // ticket_token
    String, // ticket_target
    String, // project
    String, // created_at
);

fn read_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<PeerRow> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
        row.get(6)?,
        row.get(7)?,
        row.get(8)?,
    ))
}

fn peer_from_row(row: PeerRow) -> Result<Peer, EventError> {
    let (id, key, address, name, kind_str, ticket_token, ticket_target, project_str, created_at) =
        row;

    let parsed_key: PeerKey = key
        .parse()
        .map_err(|error: PeerKeyError| EventError::Import(error.to_string()))?;
    let parsed_addr: PeerAddress = address
        .parse()
        .map_err(|error: PeerAddressError| EventError::Import(error.to_string()))?;

    let kind = kind_str
        .parse()
        .map_err(|error: PeerKindParseFailure| EventError::Import(error.to_string()))?;

    let ticket = if ticket_token.is_empty() || ticket_target.is_empty() {
        None
    } else {
        let target_ref: RefToken = ticket_target
            .parse()
            .map_err(|error: RefError| EventError::Import(error.to_string()))?;
        Some(Link::new(
            target_ref.into_inner(),
            Token::from(ticket_token),
        ))
    };

    let project = if project_str.is_empty() {
        None
    } else {
        Some(ProjectName::new(project_str))
    };

    Ok(Peer::builder()
        .id(id.parse::<PeerId>()?)
        .key(parsed_key)
        .address(parsed_addr)
        .name(PeerName::new(name))
        .kind(kind)
        .maybe_ticket(ticket)
        .maybe_project(project)
        .created_at(Timestamp::parse_str(created_at)?)
        .build())
}
