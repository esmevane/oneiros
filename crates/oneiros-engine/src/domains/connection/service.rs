use crate::*;

pub struct ConnectionService;

impl ConnectionService {
    pub async fn create(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &CreateConnection,
    ) -> Result<ConnectionResponse, ConnectionError> {
        let CreateConnection::V1(creation) = request;

        let connection = Connection::builder()
            .from_ref(creation.from_ref.clone().into_inner())
            .to_ref(creation.to_ref.clone().into_inner())
            .nature(creation.nature.clone())
            .build();
        let id = connection.id;

        let new_event = NewEvent::builder()
            .data(Events::Connection(ConnectionEvents::ConnectionCreated(
                ConnectionCreated::builder_v1()
                    .connection(connection)
                    .build()
                    .into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        let stored = ConnectionRepo::new(scope)
            .fetch(&id)
            .await?
            .ok_or(ConnectionError::NotFound(id))?;

        Ok(ConnectionResponse::ConnectionCreated(
            ConnectionCreatedResponse::builder_v1()
                .connection(stored)
                .build()
                .into(),
        ))
    }

    pub async fn get(
        scope: &Scope<AtBookmark>,
        request: &GetConnection,
    ) -> Result<ConnectionResponse, ConnectionError> {
        let GetConnection::V1(lookup) = request;
        let id = lookup.key.resolve()?;
        let connection = ConnectionRepo::new(scope)
            .fetch(&id)
            .await?
            .ok_or(ConnectionError::NotFound(id))?;
        Ok(ConnectionResponse::ConnectionDetails(
            ConnectionDetailsResponse::builder_v1()
                .connection(connection)
                .build()
                .into(),
        ))
    }

    pub async fn list(
        scope: &Scope<AtBookmark>,
        request: &ListConnections,
    ) -> Result<ConnectionResponse, ConnectionError> {
        let ListConnections::V1(listing) = request;
        let ref_json = listing
            .entity
            .as_ref()
            .map(|token| {
                serde_json::to_string(&token.clone().into_inner())
                    .map_err(|e| ConnectionError::Event(e.into()))
            })
            .transpose()?;

        let listed = ConnectionRepo::new(scope)
            .list(ref_json.as_deref(), &listing.filters)
            .await?;
        if listed.total == 0 {
            Ok(ConnectionResponse::NoConnections)
        } else {
            Ok(ConnectionResponse::Connections(
                ConnectionsResponse::builder_v1()
                    .items(listed.items)
                    .total(listed.total)
                    .build()
                    .into(),
            ))
        }
    }

    pub async fn remove(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &RemoveConnection,
    ) -> Result<ConnectionResponse, ConnectionError> {
        let RemoveConnection::V1(removal) = request;
        if ConnectionRepo::new(scope)
            .fetch(&removal.id)
            .await?
            .is_none()
        {
            return Err(ConnectionError::NotFound(removal.id));
        }

        let new_event = NewEvent::builder()
            .data(Events::Connection(ConnectionEvents::ConnectionRemoved(
                ConnectionRemoved::builder_v1()
                    .id(removal.id)
                    .build()
                    .into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        scope
            .config()
            .fetch
            .until_absent(|| async { ConnectionRepo::new(scope).get(&removal.id).await })
            .await?;

        Ok(ConnectionResponse::ConnectionRemoved(
            ConnectionRemovedResponse::builder_v1()
                .id(removal.id)
                .build()
                .into(),
        ))
    }
}
