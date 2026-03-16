use oneiros_model::*;

use crate::*;

pub struct ConnectionStore;

impl Dispatch<ConnectionRequests> for ConnectionStore {
    type Response = ConnectionResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, ConnectionRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            ConnectionRequests::CreateConnection(request) => {
                db.get_nature(&request.nature)?
                    .ok_or(NotFound::Nature(request.nature.clone()))?;

                let connection =
                    Connection::create(request.nature, request.from_ref, request.to_ref);

                let event =
                    Events::Connection(ConnectionEvents::ConnectionCreated(connection.clone()));
                context.scope.effects().emit(&event)?;

                Ok(ConnectionResponses::ConnectionCreated(connection))
            }
            ConnectionRequests::GetConnection(request) => {
                let connection = db
                    .get_connection(request.id.to_string())?
                    .ok_or(NotFound::Connection(request.id))?;
                Ok(ConnectionResponses::ConnectionFound(connection))
            }
            ConnectionRequests::ListConnections(request) => {
                let connections = match (request.nature, request.entity_ref) {
                    (Some(nature), Some(ref_token)) => {
                        db.get_nature(&nature)?
                            .ok_or(NotFound::Nature(nature.clone()))?;

                        db.list_connections_by_ref(ref_token.inner())?
                            .into_iter()
                            .filter(|c| c.nature == nature)
                            .collect()
                    }
                    (Some(nature), None) => {
                        db.get_nature(&nature)?
                            .ok_or(NotFound::Nature(nature.clone()))?;

                        db.list_connections_by_nature(&nature)?
                    }
                    (None, Some(ref_token)) => db.list_connections_by_ref(ref_token.inner())?,
                    (None, None) => db.list_connections()?,
                };

                Ok(ConnectionResponses::ConnectionsListed(connections))
            }
            ConnectionRequests::RemoveConnection(request) => {
                db.get_connection(request.id.to_string())?
                    .ok_or(NotFound::Connection(request.id))?;

                let event =
                    Events::Connection(ConnectionEvents::ConnectionRemoved(SelectConnectionById {
                        id: request.id,
                    }));
                context.scope.effects().emit(&event)?;

                Ok(ConnectionResponses::ConnectionRemoved)
            }
        }
    }
}
