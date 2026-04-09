use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = TenantResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum TenantResponse {
    Created(Response<Tenant>),
    Found(Response<Tenant>),
    Listed(Listed<Response<Tenant>>),
}
