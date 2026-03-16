use oneiros_model::*;

use crate::*;

pub struct TenantStore;

impl Dispatch<TenantRequests> for TenantStore {
    type Response = TenantResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, TenantRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            TenantRequests::GetTenant(request) => {
                let tenant = db
                    .get_tenant_by_name(&request.name)?
                    .ok_or(NotFound::Tenant(request.name))?;
                Ok(TenantResponses::TenantFound(tenant))
            }
            TenantRequests::ListTenants(_) => {
                Ok(TenantResponses::TenantsListed(db.list_tenants()?))
            }
        }
    }
}
