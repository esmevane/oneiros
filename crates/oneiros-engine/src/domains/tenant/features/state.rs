use crate::*;

pub struct TenantState;

impl TenantState {
    pub fn reduce(mut canon: SystemCanon, event: &Events) -> SystemCanon {
        if let Events::Tenant(TenantEvents::TenantCreated(tenant)) = event {
            canon.tenants.set(tenant);
        }

        canon
    }

    pub fn reducer() -> Reducer<SystemCanon> {
        Reducer::new(Self::reduce)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_tenant() {
        let canon = SystemCanon::default();
        let tenant = Tenant::builder().name("test-tenant").build();
        let event = Events::Tenant(TenantEvents::TenantCreated(tenant.clone()));

        let next = TenantState::reduce(canon, &event);

        assert_eq!(next.tenants.len(), 1);
    }
}
