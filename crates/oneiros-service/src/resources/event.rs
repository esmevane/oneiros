use oneiros_model::*;

use crate::*;

pub struct EventStore;

impl Dispatch<EventRequests> for EventStore {
    type Response = EventResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, EventRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            EventRequests::ListEvents(request) => {
                Ok(EventResponses::Listed(db.read_events(request.after)?))
            }
            EventRequests::GetEvent(request) => {
                let event = db
                    .get_event(&request.id)?
                    .ok_or(NotFound::Event(request.id))?;
                Ok(EventResponses::Found(event))
            }
            EventRequests::ImportEvents(request) => {
                let source = context.scope.source();

                for event in &request.events {
                    let event = event.clone().with_source(source);
                    db.import_event(&event)?;
                }

                let replayed = db.replay(projections::BRAIN)?;

                Ok(EventResponses::Imported(ImportResponse {
                    imported: request.events.len(),
                    replayed,
                }))
            }
            EventRequests::ReplayEvents(_) => {
                let count = db.replay(projections::BRAIN)?;
                Ok(EventResponses::Replayed(ReplayResponse { replayed: count }))
            }
            EventRequests::ExportEvents(_) => Ok(EventResponses::Exported(db.read_events(None)?)),
        }
    }
}
