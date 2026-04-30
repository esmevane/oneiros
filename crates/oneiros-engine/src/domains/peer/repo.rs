use rusqlite::params;

use crate::*;

/// Peer read model — async queries against the system context.
pub struct PeerRepo<'a> {
    scope: &'a Scope<AtHost>,
}

impl<'a> PeerRepo<'a> {
    pub fn new(scope: &'a Scope<AtHost>) -> Self {
        Self { scope }
    }

    pub async fn get(&self, id: PeerId) -> Result<Option<Peer>, EventError> {
        let db = self.scope.host_db()?;
        let mut statement =
            db.prepare("select id, key, address, name, created_at from peers where id = ?1")?;

        let raw = statement.query_row(params![id.to_string()], |row| {
            let id: String = row.get(0)?;
            let key: String = row.get(1)?;
            let address: String = row.get(2)?;
            let name: String = row.get(3)?;
            let created_at: String = row.get(4)?;

            Ok((id, key, address, name, created_at))
        });

        match raw {
            Ok((id, key, address, name, created_at)) => {
                Ok(Some(peer_from_columns(id, key, address, name, created_at)?))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub async fn list(&self, filters: &SearchFilters) -> Result<Listed<Peer>, EventError> {
        let db = self.scope.host_db()?;

        let total = {
            let mut stmt = db.prepare("SELECT COUNT(*) FROM peers")?;
            stmt.query_row([], |row| row.get::<_, usize>(0))?
        };

        let select_sql = "SELECT id, key, address, name, created_at \
                          FROM peers ORDER BY created_at DESC LIMIT ?1 OFFSET ?2";
        let mut statement = db.prepare(select_sql)?;

        let raw: Vec<(String, String, String, String, String)> = statement
            .query_map(rusqlite::params![filters.limit, filters.offset], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut peers = vec![];

        for (id, key, address, name, created_at) in raw {
            peers.push(peer_from_columns(id, key, address, name, created_at)?);
        }

        Ok(Listed::new(peers, total))
    }
}

fn peer_from_columns(
    id: String,
    key: String,
    address: String,
    name: String,
    created_at: String,
) -> Result<Peer, EventError> {
    let parsed_key: PeerKey = key
        .parse()
        .map_err(|e: PeerKeyError| EventError::Import(e.to_string()))?;
    let parsed_addr: PeerAddress = address
        .parse()
        .map_err(|e: PeerAddressError| EventError::Import(e.to_string()))?;

    Ok(Peer::builder()
        .id(id.parse::<PeerId>()?)
        .key(parsed_key)
        .address(parsed_addr)
        .name(PeerName::new(name))
        .created_at(Timestamp::parse_str(created_at)?)
        .build())
}
