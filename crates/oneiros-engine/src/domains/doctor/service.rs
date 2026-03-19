use crate::*;

pub struct DoctorService;

impl DoctorService {
    pub fn check(ctx: &SystemContext) -> Vec<DoctorResponse> {
        let mut results = Vec::new();

        let tenants = ctx
            .with_db(|conn| TenantRepo::new(conn).list())
            .unwrap_or_default();

        if tenants.is_empty() {
            results.push(DoctorResponse::NotInitialized);
            return results;
        }

        results.push(DoctorResponse::Initialized);
        results.push(DoctorResponse::DatabaseOk("system.db".to_string()));

        let event_count = ctx
            .with_db(|conn| {
                conn.query_row("SELECT COUNT(*) FROM events", [], |row| {
                    row.get::<_, i64>(0)
                })
            })
            .unwrap_or(0);

        results.push(DoctorResponse::EventLogReady(event_count));

        results
    }
}
