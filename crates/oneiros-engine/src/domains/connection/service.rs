use crate::*;

pub(crate) struct ConnectionService;

impl ConnectionService {
    pub(crate) async fn create(
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

        let new_event = NewEvent::builder()
            .data(Events::Connection(ConnectionEvents::ConnectionCreated(
                ConnectionCreated::builder_v1()
                    .connection(connection.clone())
                    .build()
                    .into(),
            )))
            .build();

        mailbox.tell(scope.append(new_event.clone()));
        scope.ensure_persisted(&new_event).await?;

        Ok(ConnectionResponse::ConnectionCreated(
            ConnectionCreatedResponse::builder_v1()
                .connection(connection)
                .build()
                .into(),
        ))
    }

    pub(crate) async fn get(
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

    pub(crate) async fn list(
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

    pub(crate) async fn remove(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &RemoveConnection,
    ) -> Result<ConnectionResponse, ConnectionError> {
        let RemoveConnection::V1(removal) = request;
        let id = removal.key.resolve()?;
        if ConnectionRepo::new(scope).fetch(&id).await?.is_none() {
            return Err(ConnectionError::NotFound(id));
        }

        let new_event = NewEvent::builder()
            .data(Events::Connection(ConnectionEvents::ConnectionRemoved(
                ConnectionRemoved::builder_v1().id(id).build().into(),
            )))
            .build();

        mailbox.tell(scope.append(new_event.clone()));
        scope.ensure_persisted(&new_event).await?;

        Ok(ConnectionResponse::ConnectionRemoved(
            ConnectionRemovedResponse::builder_v1()
                .id(id)
                .build()
                .into(),
        ))
    }
}
