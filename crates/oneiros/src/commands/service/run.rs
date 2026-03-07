use std::net::SocketAddr;
use std::sync::Arc;

use clap::Args;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};
use oneiros_service::ServiceState;
use oneiros_trust::{LocalCa, ResolvedMode, SecureServer, build_acme_state, resolve_mode};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RunServiceOutcomes {
    #[outcome(message("Service starting on {0}."))]
    ServiceStarting(SocketAddr),
    #[outcome(message("Service stopped."))]
    ServiceStopped,
}

#[derive(Clone, Args)]
pub struct RunService;

impl RunService {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<RunServiceOutcomes>, ServiceCommandError> {
        let mut outcomes = Outcomes::new();

        if !context.is_initialized() {
            return Err(ServiceCommandError::NotInitialized);
        }

        let database = context.database()?;
        let addr = context.config().service_addr();

        let tenant_id: TenantId = database
            .get_tenant_id()?
            .ok_or(ServiceCommandError::MissingId)?
            .parse()?;

        let actor_id: ActorId = database
            .get_actor_id(tenant_id.to_string())?
            .ok_or(ServiceCommandError::MissingId)?
            .parse()?;

        let source = Source {
            actor_id,
            tenant_id,
        };

        outcomes.emit(RunServiceOutcomes::ServiceStarting(addr));

        let config = context.config();
        let trust_config = &config.trust;
        let hostname = &config.service.host;
        let data_dir = context.data_dir();

        let resolved_mode = resolve_mode(&trust_config.mode, hostname);
        let secure_server = match &resolved_mode {
            ResolvedMode::Off => {
                tracing::info!("TLS disabled (trust mode is Off) — serving plain TCP");
                None
            }
            ResolvedMode::Local => {
                let trust_dir = data_dir.join("trust");
                match LocalCa::init(&trust_dir).and_then(|ca| ca.issue_leaf(hostname)) {
                    Ok(leaf) => match SecureServer::local(&leaf) {
                        Ok(server) => {
                            tracing::info!(?resolved_mode, "TLS configured for service");
                            Some(server)
                        }
                        Err(e) => {
                            tracing::warn!("Failed to configure TLS: {e} — falling back to plain TCP");
                            None
                        }
                    },
                    Err(e) => {
                        tracing::warn!("Failed to configure TLS: {e} — falling back to plain TCP");
                        None
                    }
                }
            }
            ResolvedMode::Acme => {
                let domains = vec![hostname.clone()];
                match build_acme_state(&trust_config.acme, domains, data_dir) {
                    Ok(state) => {
                        tracing::info!(?resolved_mode, "TLS configured for service");
                        Some(SecureServer::Acme(state))
                    }
                    Err(e) => {
                        tracing::warn!("Failed to configure ACME: {e} — falling back to plain TCP");
                        None
                    }
                }
            }
        };

        let state = Arc::new(ServiceState::new(
            database,
            data_dir.to_path_buf(),
            source,
        ));

        oneiros_http::serve(state, addr, secure_server).await?;

        outcomes.emit(RunServiceOutcomes::ServiceStopped);

        Ok(outcomes)
    }
}
