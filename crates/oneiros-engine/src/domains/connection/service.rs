use crate::*;

pub struct ConnectionService;

impl ConnectionService {
    pub async fn create(
        context: &ProjectContext,
        CreateConnection {
            from_ref,
            to_ref,
            nature,
        }: &CreateConnection,
    ) -> Result<ConnectionResponse, ConnectionError> {
        let from = from_ref.clone().into_inner();
        let to = to_ref.clone().into_inner();

        let connection = Connection::Current(
            Connection::build_v1()
                .from_ref(from)
                .to_ref(to)
                .nature(nature.clone())
                .build(),
        );

        context
            .emit(ConnectionEvents::ConnectionCreated(connection.clone()))
            .await?;
        let ref_token = RefToken::new(Ref::connection(connection.id()));
        Ok(ConnectionResponse::ConnectionCreated(
            Response::new(connection).with_ref_token(ref_token),
        ))
    }

    pub async fn get(
        context: &ProjectContext,
        selector: &GetConnection,
    ) -> Result<ConnectionResponse, ConnectionError> {
        let id = selector.key.resolve()?;
        let connection = ConnectionRepo::new(context)
            .get(&id)
            .await?
            .ok_or(ConnectionError::NotFound(id))?;
        let ref_token = RefToken::new(Ref::connection(connection.id()));
        Ok(ConnectionResponse::ConnectionDetails(
            Response::new(connection).with_ref_token(ref_token),
        ))
    }

    pub async fn list(
        context: &ProjectContext,
        ListConnections { entity, filters }: &ListConnections,
    ) -> Result<ConnectionResponse, ConnectionError> {
        let ref_json = entity
            .as_ref()
            .map(|token| {
                serde_json::to_string(&token.clone().into_inner())
                    .map_err(|e| ConnectionError::Event(e.into()))
            })
            .transpose()?;

        let listed = ConnectionRepo::new(context)
            .list(ref_json.as_deref(), filters)
            .await?;
        if listed.total == 0 {
            Ok(ConnectionResponse::NoConnections)
        } else {
            Ok(ConnectionResponse::Connections(listed.map(|c| {
                let ref_token = RefToken::new(Ref::connection(c.id()));
                Response::new(c).with_ref_token(ref_token)
            })))
        }
    }

    pub async fn remove(
        context: &ProjectContext,
        selector: &RemoveConnection,
    ) -> Result<ConnectionResponse, ConnectionError> {
        if ConnectionRepo::new(context)
            .get(&selector.id)
            .await?
            .is_none()
        {
            return Err(ConnectionError::NotFound(selector.id));
        }

        context
            .emit(ConnectionEvents::ConnectionRemoved(
                ConnectionRemoved::Current(ConnectionRemovedV1 { id: selector.id }),
            ))
            .await?;
        Ok(ConnectionResponse::ConnectionRemoved(selector.id))
    }
}
