use crate::*;

enum McpLiveResult {
    Reachable,
    TokenRejected(String),
}

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

        // Service check — run before MCP so the live check can gate on it
        let service_status = HostService::status(config).await;
        match service_status {
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

        // MCP config check — with live token validation when the service is up
        if McpConfigService::is_configured() {
            checks.push(DoctorCheck::McpConfigured);
            let service_is_running = matches!(service_status, HostResponse::ServiceRunning(_));
            if service_is_running {
                if let Some(mcp_config) = McpConfigService::read_config(config) {
                    Self::check_mcp_live(&mcp_config, &mut checks).await;
                }
            } else {
                checks.push(DoctorCheck::McpNotVerified);
            }
        } else {
            checks.push(DoctorCheck::McpMissing);
        }

        DoctorResponse::CheckupStatus(
            CheckupStatusResponse::builder_v1()
                .checks(checks)
                .build()
                .into(),
        )
    }

    async fn check_mcp_live(config: &McpLiveConfig, checks: &mut Vec<DoctorCheck>) {
        let client = reqwest::Client::new();
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": { "name": "oneiros-doctor", "version": "0.1.0" }
            }
        });

        let result = client
            .post(&config.url)
            .header("Accept", "application/json, text/event-stream")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", config.token))
            .json(&body)
            .send()
            .await;

        let resp = match result {
            Ok(r) if r.status().is_success() => r,
            Ok(r) => {
                checks.push(DoctorCheck::McpUnreachable(DiagnosticMessage::new(
                    format!("HTTP {}", r.status().as_u16()),
                )));
                return;
            }
            Err(e) => {
                checks.push(DoctorCheck::McpUnreachable(DiagnosticMessage::new(
                    e.to_string(),
                )));
                return;
            }
        };

        let body_text = match resp.text().await {
            Ok(t) => t,
            Err(e) => {
                checks.push(DoctorCheck::McpUnreachable(DiagnosticMessage::new(
                    e.to_string(),
                )));
                return;
            }
        };

        match Self::parse_mcp_initialize_response(&body_text) {
            Ok(McpLiveResult::Reachable) => {
                checks.push(DoctorCheck::McpReachable);
            }
            Ok(McpLiveResult::TokenRejected(msg)) => {
                checks.push(DoctorCheck::McpTokenRejected(DiagnosticMessage::new(msg)));
            }
            Err(e) => {
                checks.push(DoctorCheck::McpUnreachable(DiagnosticMessage::new(e)));
            }
        }
    }

    /// Parse an SSE-wrapped JSON-RPC initialize response. Returns
    /// `Reachable` on a valid result, `TokenRejected` on -32602 with an
    /// "Invalid" message, or an error string for other parse failures.
    fn parse_mcp_initialize_response(body: &str) -> Result<McpLiveResult, String> {
        let json_str = body
            .lines()
            .filter(|line| line.starts_with("data:"))
            .map(|line| line.strip_prefix("data:").unwrap().trim())
            .find(|s| !s.is_empty())
            .ok_or_else(|| "no data: line in SSE response".to_string())?;

        let parsed: serde_json::Value =
            serde_json::from_str(json_str).map_err(|e| e.to_string())?;

        if let Some(result) = parsed.get("result")
            && result.is_object()
        {
            return Ok(McpLiveResult::Reachable);
        }

        let error = parsed
            .get("error")
            .ok_or_else(|| "no result or error in JSON-RPC response".to_string())?;

        let code = error.get("code").and_then(|c| c.as_i64()).unwrap_or(0);

        let message = error.get("message").and_then(|m| m.as_str()).unwrap_or("");

        if code == -32602 && message.contains("Invalid") {
            return Ok(McpLiveResult::TokenRejected(message.to_string()));
        }

        Err(format!("JSON-RPC error {code}: {message}"))
    }
}
