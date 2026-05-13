use crate::*;

pub(crate) struct TenantState;

impl TenantState {
    pub(crate) fn reduce(mut canon: HostCanon, event: &Events) -> HostCanon {
        if let Events::Tenant(tenant_event) = event
            && let Some(tenant) = tenant_event.maybe_tenant()
        {
            canon.tenants.set(&tenant);
        }

        canon
    }

    pub(crate) fn reducer() -> Reducer<HostCanon> {
        Reducer::new(Self::reduce)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_tenant() {
        let canon = HostCanon::default();
        let tenant = Tenant::builder()
            .name(TenantName::new("test-tenant"))
            .build();
        let event = Events::Tenant(TenantEvents::TenantCreated(
            TenantCreated::builder_v1().tenant(tenant).build().into(),
        ));

        let next = TenantState::reduce(canon, &event);

        assert_eq!(next.tenants.len(), 1);
    }
}
