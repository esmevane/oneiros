use crate::*;

pub struct DoctorService;

impl DoctorService {
    pub fn check(ctx: &SystemContext) -> DoctorResponse {
        let mut checks = Vec::new();

        let tenants = ctx
            .with_db(|conn| TenantRepo::new(conn).list())
            .unwrap_or_default();

        if tenants.is_empty() {
            checks.push(DoctorCheck::NotInitialized);
            return DoctorResponse::CheckupStatus(checks);
        }

        checks.push(DoctorCheck::Initialized);
        checks.push(DoctorCheck::DatabaseOk("system.db".to_string()));

        let event_count = ctx
            .with_db(|conn| {
                conn.query_row("SELECT COUNT(*) FROM events", [], |row| {
                    row.get::<_, i64>(0)
                })
            })
            .unwrap_or(0);

        checks.push(DoctorCheck::EventLogReady(event_count));

        DoctorResponse::CheckupStatus(checks)
    }
}
