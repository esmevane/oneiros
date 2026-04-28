use crate::*;

pub struct TenantView {
    response: TenantResponse,
}

impl TenantView {
    pub fn new(response: TenantResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<TenantResponse> {
        match self.response {
            TenantResponse::Created(TenantCreatedResponse::V1(created)) => {
                let prompt =
                    Confirmation::new("Tenant", created.tenant.name.to_string(), "created")
                        .to_string();
                Rendered::new(
                    TenantResponse::Created(TenantCreatedResponse::V1(created)),
                    prompt,
                    String::new(),
                )
            }
            TenantResponse::Found(TenantFoundResponse::V1(found)) => {
                let prompt = Detail::new(found.tenant.name.to_string())
                    .field("id:", found.tenant.id.to_string())
                    .to_string();
                Rendered::new(
                    TenantResponse::Found(TenantFoundResponse::V1(found)),
                    prompt,
                    String::new(),
                )
            }
            TenantResponse::Listed(TenantsResponse::V1(listed)) => {
                let mut table =
                    Table::new(vec![Column::key("name", "Name"), Column::key("id", "ID")]);
                for tenant in &listed.items {
                    table.push_row(vec![tenant.name.to_string(), tenant.id.to_string()]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.items.len(), listed.total).muted(),
                );
                Rendered::new(
                    TenantResponse::Listed(TenantsResponse::V1(listed)),
                    prompt,
                    String::new(),
                )
            }
        }
    }
}
