//! Tenant view — presentation authority for the tenant domain.
//!
//! Maps tenant responses into shared view primitives (Table, Detail,
//! Confirmation). The domain knows its own shape; the rendering
//! layer decides how to display it.

use crate::*;

pub struct TenantView;

impl TenantView {
    /// Table of tenants with standard columns.
    pub fn table(tenants: &Listed<Response<Tenant>>) -> Table {
        let mut table = Table::new(vec![Column::key("name", "Name"), Column::key("id", "ID")]);

        for wrapped in &tenants.items {
            let tenant = &wrapped.data;
            table.push_row(vec![tenant.name.to_string(), tenant.id.to_string()]);
        }

        table
    }

    /// Detail view for a single tenant.
    pub fn detail(tenant: &Tenant) -> Detail {
        Detail::new(tenant.name.to_string()).field("id:", tenant.id.to_string())
    }

    /// Confirmation for a mutation.
    pub fn confirmed(verb: &str, name: &TenantName) -> Confirmation {
        Confirmation::new("Tenant", name.to_string(), verb)
    }
}
