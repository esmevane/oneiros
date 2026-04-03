use crate::*;

pub struct TenantService;

impl TenantService {
    pub async fn create(
        context: &SystemContext,
        CreateTenant { name }: &CreateTenant,
    ) -> Result<TenantResponse, TenantError> {
        let tenant = Tenant::builder().name(name.clone()).build();

        context
            .emit(TenantEvents::TenantCreated(tenant.clone()))
            .await?;
        let ref_token = RefToken::new(Ref::tenant(tenant.id));
        Ok(TenantResponse::Created(
            Response::new(tenant).with_ref_token(ref_token),
        ))
    }

    pub async fn get(
        context: &SystemContext,
        selector: &GetTenant,
    ) -> Result<TenantResponse, TenantError> {
        let tenant = TenantRepo::new(context)
            .get(&selector.id)
            .await?
            .ok_or_else(|| TenantError::NotFound(selector.id))?;
        let ref_token = RefToken::new(Ref::tenant(tenant.id));
        Ok(TenantResponse::Found(
            Response::new(tenant).with_ref_token(ref_token),
        ))
    }

    pub async fn list(
        context: &SystemContext,
        ListTenants { filters }: &ListTenants,
    ) -> Result<TenantResponse, TenantError> {
        let listed = TenantRepo::new(context).list(filters).await?;
        Ok(TenantResponse::Listed(listed.map(|t| {
            let ref_token = RefToken::new(Ref::tenant(t.id));
            Response::new(t).with_ref_token(ref_token)
        })))
    }
}
