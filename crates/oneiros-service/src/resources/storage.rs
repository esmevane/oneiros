use oneiros_model::*;

use crate::*;

pub struct StorageStore;

impl Dispatch<StorageRequests> for StorageStore {
    type Response = StorageResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, StorageRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            StorageRequests::ListStorage(_) => {
                Ok(StorageResponses::StorageListed(db.list_storage()?))
            }
            StorageRequests::GetStorage(request) => {
                let entry = db
                    .get_storage(&request.key)?
                    .ok_or(NotFound::Storage(request.key))?;
                Ok(StorageResponses::StorageFound(entry))
            }
            StorageRequests::RemoveStorage(request) => {
                let event = Events::Storage(StorageEvents::StorageRemoved(SelectStorageByKey {
                    key: request.key,
                }));
                context.scope.effects().emit(&event)?;
                Ok(StorageResponses::StorageRemoved)
            }
        }
    }
}
