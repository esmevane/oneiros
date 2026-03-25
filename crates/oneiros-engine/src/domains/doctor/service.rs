use crate::*;

pub struct DoctorService;

impl DoctorService {
    pub async fn check(context: &SystemContext) -> DoctorResponse {
        let mut checks = Vec::new();

        let db = match context.db() {
            Ok(db) => db,
            Err(_) => {
                checks.push(DoctorCheck::NotInitialized);
                return DoctorResponse::CheckupStatus(checks);
            }
        };

        let tenants = TenantRepo::new(context).list().await.unwrap_or_default();

        if tenants.is_empty() {
            checks.push(DoctorCheck::NotInitialized);
            return DoctorResponse::CheckupStatus(checks);
        }

        checks.push(DoctorCheck::Initialized);
        checks.push(DoctorCheck::DatabaseOk(DatabaseLabel::new("system.db")));

        let event_count = db
            .query_row("select count(*) from events", [], |row| {
                row.get::<_, i64>(0)
            })
            .unwrap_or(0);

        checks.push(DoctorCheck::EventLogReady(LogEventCount::new(event_count)));

        DoctorResponse::CheckupStatus(checks)
    }
}
