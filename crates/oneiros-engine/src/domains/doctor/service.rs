use crate::*;

pub(crate) struct DoctorService;

impl DoctorService {
    pub(crate) async fn check(config: &Config) -> DoctorResponse {
        let mut checks = Vec::new();

        // System checks
        let context = config.system();

        let db = match context.db() {
            Ok(db) => db,
            Err(_) => {
                checks.push(DoctorCheck::NotInitialized);
                return DoctorResponse::CheckupStatus(checks);
            }
        };

        let all_filters = SearchFilters {
            limit: Limit(usize::MAX),
            offset: Offset(0),
        };
        let tenant_count = TenantRepo::new(&context)
            .list(&all_filters)
            .await
            .map(|l| l.total)
            .unwrap_or(0);

        if tenant_count == 0 {
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

        // Brain check
        let brain_name = config.brain.clone();

        match config.brain_db() {
            Ok(brain_db) => {
                let brain_events = brain_db
                    .query_row("select count(*) from events", [], |row| {
                        row.get::<_, i64>(0)
                    })
                    .unwrap_or(-1);

                if brain_events >= 0 {
                    checks.push(DoctorCheck::BrainExists(brain_name.clone()));
                    checks.push(DoctorCheck::DatabaseOk(DatabaseLabel::new("brain.db")));

                    // Vocabulary check — look for any levels
                    let has_levels = brain_db
                        .query_row("select count(*) from levels", [], |row| {
                            row.get::<_, i64>(0)
                        })
                        .unwrap_or(0);

                    if has_levels > 0 {
                        checks.push(DoctorCheck::VocabularySeeded);
                    } else {
                        checks.push(DoctorCheck::VocabularyMissing);
                    }

                    // Agent check — look for governor.process
                    let has_governor = brain_db
                        .query_row(
                            "select count(*) from agents where name = 'governor.process'",
                            [],
                            |row| row.get::<_, i64>(0),
                        )
                        .unwrap_or(0);

                    if has_governor > 0 {
                        checks.push(DoctorCheck::AgentsSeeded);
                    } else {
                        checks.push(DoctorCheck::AgentsMissing);
                    }
                } else {
                    checks.push(DoctorCheck::BrainMissing(brain_name));
                }
            }
            Err(_) => {
                checks.push(DoctorCheck::BrainMissing(brain_name));
            }
        }

        // MCP config check
        if McpConfigService::is_configured() {
            checks.push(DoctorCheck::McpConfigured);
        } else {
            checks.push(DoctorCheck::McpMissing);
        }

        // Service check
        match ServiceService::status(config).await {
            ServiceResponse::ServiceRunning(_) => {
                checks.push(DoctorCheck::ServiceRunning);
            }
            ServiceResponse::ServiceStopped => {
                checks.push(DoctorCheck::ServiceStopped);
            }
            _ => {
                checks.push(DoctorCheck::ServiceNotInstalled);
            }
        }

        DoctorResponse::CheckupStatus(checks)
    }
}
