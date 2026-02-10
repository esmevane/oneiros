use std::future::Future;

use crate::*;

pub trait ServiceClient: Send + Sync {
    fn create_brain(
        &self,
        request: CreateBrainRequest,
    ) -> impl Future<Output = Result<BrainInfo, Error>> + Send;

    fn health(&self) -> impl Future<Output = Result<(), Error>> + Send;
}
