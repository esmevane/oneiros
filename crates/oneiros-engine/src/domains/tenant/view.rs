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
            TenantResponse::Created(wrapped) => {
                let prompt =
                    Confirmation::new("Tenant", wrapped.data.name().to_string(), "created")
                        .to_string();
                Rendered::new(TenantResponse::Created(wrapped), prompt, String::new())
            }
            TenantResponse::Found(wrapped) => {
                let prompt = Detail::new(wrapped.data.name().to_string())
                    .field("id:", wrapped.data.id().to_string())
                    .to_string();
                Rendered::new(TenantResponse::Found(wrapped), prompt, String::new())
            }
            TenantResponse::Listed(listed) => {
                let mut table =
                    Table::new(vec![Column::key("name", "Name"), Column::key("id", "ID")]);
                for wrapped in &listed.items {
                    let tenant = &wrapped.data;
                    table.push_row(vec![tenant.name().to_string(), tenant.id().to_string()]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                );
                Rendered::new(TenantResponse::Listed(listed), prompt, String::new())
            }
        }
    }
}
