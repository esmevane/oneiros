use crate::*;

pub(crate) struct DoctorService;

impl DoctorService {
    pub(crate) async fn check(config: &Config) -> DoctorResponse {
        let mut checks = Vec::new();

        // Compose host-tier scope. Failure here means we don't have
        // host substrate at all — strangler bridge still produces
        // today's HostLog shape until consumers move to Scope.
        let scope = match ComposeScope::new(config.clone()).host() {
            Ok(scope) => scope,
            Err(_) => {
                checks.push(DoctorCheck::NotInitialized);
                return DoctorResponse::CheckupStatus(
                    CheckupStatusResponse::builder_v1()
                        .checks(checks)
                        .build()
                        .into(),
                );
            }
        };

        let db = match HostDb::open(&scope).await {
            Ok(db) => db,
            Err(_) => {
                checks.push(DoctorCheck::NotInitialized);
                return DoctorResponse::CheckupStatus(
                    CheckupStatusResponse::builder_v1()
                        .checks(checks)
                        .build()
                        .into(),
                );
            }
        };

        let all_filters = SearchFilters {
            limit: Limit(usize::MAX),
            offset: Offset(0),
        };
        let tenant_count = TenantRepo::new(&scope)
            .list(&all_filters)
            .await
            .map(|l| l.total)
            .unwrap_or(0);

        if tenant_count == 0 {
            checks.push(DoctorCheck::NotInitialized);
            return DoctorResponse::CheckupStatus(
                CheckupStatusResponse::builder_v1()
                    .checks(checks)
                    .build()
                    .into(),
            );
        }

        checks.push(DoctorCheck::Initialized);
        checks.push(DoctorCheck::DatabaseOk(DatabaseLabel::new("host.db")));

        // Host keypair check — identity for distribution
        if HostKey::new(config.platform()).path().exists() {
            checks.push(DoctorCheck::HostKeyOk);
        } else {
            checks.push(DoctorCheck::HostKeyMissing);
        }

        let event_count = db
            .query_row("select count(*) from events", [], |row| {
                row.get::<_, i64>(0)
            })
            .unwrap_or(0);

        checks.push(DoctorCheck::EventLogReady(LogEventCount::new(event_count)));

        // Project check
        let project_name = config.project.clone();

        match config.bookmark_conn() {
            Ok(project_db) => {
                let project_events = project_db
                    .query_row("select count(*) from events.events", [], |row| {
                        row.get::<_, i64>(0)
                    })
                    .unwrap_or(-1);

                if project_events >= 0 {
                    checks.push(DoctorCheck::ProjectExists(project_name.clone()));
                    checks.push(DoctorCheck::DatabaseOk(DatabaseLabel::new("events.db")));

                    // Vocabulary check — look for any levels
                    let has_levels = project_db
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
                    let has_governor = project_db
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
                    checks.push(DoctorCheck::ProjectMissing(project_name));
                }
            }
            Err(_) => {
                checks.push(DoctorCheck::ProjectMissing(project_name));
            }
        }

        // MCP config check
        if McpConfigService::is_configured() {
            checks.push(DoctorCheck::McpConfigured);
        } else {
            checks.push(DoctorCheck::McpMissing);
        }

        // Service check
        match HostService::status(config).await {
            HostResponse::ServiceRunning(_) => {
                checks.push(DoctorCheck::ServiceRunning);
            }
            HostResponse::ServiceStopped => {
                checks.push(DoctorCheck::ServiceStopped);
            }
            _ => {
                checks.push(DoctorCheck::ServiceNotInstalled);
            }
        }

        DoctorResponse::CheckupStatus(
            CheckupStatusResponse::builder_v1()
                .checks(checks)
                .build()
                .into(),
        )
    }
}
