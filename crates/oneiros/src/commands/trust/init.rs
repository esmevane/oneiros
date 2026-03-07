use clap::Args;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};
use oneiros_trust::{LocalCa, ResolvedMode, SystemTrustStore, TrustMode, TrustStoreBackend, ca_fingerprint, resolve_mode};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum TrustInitError {
    #[error("Trust error: {0}")]
    Trust(#[from] oneiros_trust::TrustError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(#[from] oneiros_db::DatabaseError),

    #[error("System not initialized. Run `oneiros system init` first.")]
    NotInitialized,

    #[error("Malformed tenant or actor ID in database.")]
    MalformedId(#[from] oneiros_model::IdParseError),

    #[error("Missing tenant or actor ID in database.")]
    MissingId,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum TrustInitOutcomes {
    #[outcome(message("Trust mode set to {0}."))]
    ModeSelected(String),
    #[outcome(message("Local CA created."))]
    CaCreated,
    #[outcome(message("Leaf certificate issued for '{0}'."))]
    LeafIssued(String),
    #[outcome(message("Root CA installed to system trust store."))]
    TrustStoreInstalled,
    #[outcome(message("Skipped system trust store installation."))]
    TrustStoreSkipped,
    #[outcome(
        message("Could not install root CA to system trust store: {0}"),
        level = "warn"
    )]
    TrustStoreInstallFailed(String),
    #[outcome(message("Trust initialized."))]
    TrustInitialized,
}

#[derive(Clone, Args)]
pub struct TrustInit {
    /// Trust mode: auto, local, acme, or off.
    #[arg(long, short, default_value = "auto")]
    mode: String,

    /// Install the root CA into the system trust store.
    /// On macOS this will prompt for keychain access.
    #[arg(long)]
    install: bool,

    /// Skip interactive prompts, use defaults.
    #[arg(short, long)]
    yes: bool,
}

impl TrustInit {
    /// Construct a `TrustInit` suitable for programmatic invocation from another
    /// command. Uses `auto` mode, skips trust-store installation, and suppresses
    /// interactive prompts so the caller controls the interaction surface.
    pub fn auto_quiet() -> Self {
        Self {
            mode: "auto".to_string(),
            install: false,
            yes: true,
        }
    }

    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<TrustInitOutcomes>, TrustInitError> {
        let mut outcomes = Outcomes::new();

        if !context.is_initialized() {
            return Err(TrustInitError::NotInitialized);
        }

        let database = context.database()?;

        let tenant_id: TenantId = database
            .get_tenant_id()?
            .ok_or(TrustInitError::MissingId)?
            .parse()?;

        let actor_id: ActorId = database
            .get_actor_id(tenant_id.to_string())?
            .ok_or(TrustInitError::MissingId)?
            .parse()?;

        let source = Source {
            actor_id,
            tenant_id,
        };

        let config = context.config();
        let hostname = &config.service.host;

        // Resolve the desired mode
        let mode = match self.mode.as_str() {
            "auto" => TrustMode::Auto,
            "local" => TrustMode::Local,
            "acme" => TrustMode::Acme,
            "off" => TrustMode::Off,
            other => {
                tracing::warn!("Unknown trust mode '{other}', using 'auto'");
                TrustMode::Auto
            }
        };

        outcomes.emit(TrustInitOutcomes::ModeSelected(format!("{mode:?}")));

        let event = Events::Trust(TrustEvents::TrustModeConfigured(TrustModeConfigured {
            mode: format!("{mode:?}"),
        }));
        let known = Event::create(event, source);
        database.log_event(&known, projections::SYSTEM)?;

        // Build a TrustConfig with the selected mode
        let mut trust_config = config.trust.clone();
        trust_config.mode = mode;

        let resolved = resolve_mode(&trust_config.mode, hostname);
        let data_dir = context.data_dir();
        let trust_dir = data_dir.join("trust");

        if resolved == ResolvedMode::Local {
            std::fs::create_dir_all(&trust_dir)?;
            let ca = LocalCa::init(&trust_dir)?;

            outcomes.emit(TrustInitOutcomes::CaCreated);

            let root_fingerprint = ca_fingerprint(ca.root_cert_pem())
                .unwrap_or_else(|| "unknown".to_string());

            let event =
                Events::Trust(TrustEvents::TrustCaInitialized(TrustCaInitialized {
                    root_fingerprint,
                    root_storage_key: None,
                }));
            let known = Event::create(event, source);
            database.log_event(&known, projections::SYSTEM)?;

            ca.issue_leaf(hostname)?;

            outcomes.emit(TrustInitOutcomes::LeafIssued(hostname.to_string()));

            // not_after is not surfaced on LeafCert; use a placeholder for now.
            let event = Events::Trust(TrustEvents::TrustLeafIssued(TrustLeafIssued {
                hostname: hostname.to_string(),
                not_after: "unknown".to_string(),
            }));
            let known = Event::create(event, source);
            database.log_event(&known, projections::SYSTEM)?;

            // Trust store installation — only if explicitly requested or interactively approved
            let should_install = self.install || (!self.yes && self.prompt_install(context));

            if should_install {
                match install_trust_store(&trust_dir, ca.root_cert_pem(), &SystemTrustStore) {
                    Ok(()) => {
                        outcomes.emit(TrustInitOutcomes::TrustStoreInstalled);

                        let event = Events::Trust(TrustEvents::TrustStoreInstalled);
                        let known = Event::create(event, source);
                        database.log_event(&known, projections::SYSTEM)?;
                    }
                    Err(err) => {
                        let reason = err.to_string();
                        outcomes.emit(TrustInitOutcomes::TrustStoreInstallFailed(reason.clone()));

                        let event = Events::Trust(TrustEvents::TrustStoreInstallFailed(
                            TrustStoreInstallFailed { reason },
                        ));
                        let known = Event::create(event, source);
                        database.log_event(&known, projections::SYSTEM)?;
                    }
                }
            } else {
                outcomes.emit(TrustInitOutcomes::TrustStoreSkipped);
            }
        } else {
            outcomes.emit(TrustInitOutcomes::TrustStoreSkipped);
        }

        // Seed peers from config
        for peer in &config.trust.peers {
            let event = Events::Trust(TrustEvents::TrustPeerAccepted(TrustPeerAccepted {
                endpoint: peer.endpoint.clone(),
                fingerprint: peer.fingerprint.clone(),
            }));
            let known = Event::create(event, source);
            database.log_event(&known, projections::SYSTEM)?;
        }

        for insecure in &config.trust.insecure {
            let event =
                Events::Trust(TrustEvents::TrustPeerInsecureAllowed(TrustPeerInsecureAllowed {
                    endpoint: insecure.endpoint.clone(),
                    reason: Some(insecure.reason.clone()),
                }));
            let known = Event::create(event, source);
            database.log_event(&known, projections::SYSTEM)?;
        }

        outcomes.emit(TrustInitOutcomes::TrustInitialized);

        Ok(outcomes)
    }

    fn prompt_install(&self, _context: &Context) -> bool {
        if !std::io::IsTerminal::is_terminal(&std::io::stdin()) {
            return false;
        }

        let message =
            "Install root CA to system trust store?\n  \
             This lets browsers and tools trust your local certs without warnings.\n  \
             You may be prompted for your system password.";

        inquire::Confirm::new(message)
            .with_default(false)
            .prompt()
            .unwrap_or(false)
    }
}

// ---------------------------------------------------------------------------
// Module-level helpers
// ---------------------------------------------------------------------------

/// Write the root CA PEM to disk and install it via `backend`.
///
/// Ensures the cert file exists under `trust_dir/certs/root.cert.pem` before
/// handing the path to the backend, matching the layout [`LocalCa::init`]
/// establishes.
fn install_trust_store(
    trust_dir: &std::path::Path,
    root_cert_pem: &str,
    backend: &dyn TrustStoreBackend,
) -> Result<(), oneiros_trust::TrustError> {
    let root_cert_path = trust_dir.join("certs").join("root.cert.pem");
    if !root_cert_path.exists() {
        std::fs::write(&root_cert_path, root_cert_pem)
            .map_err(oneiros_trust::TrustError::Io)?;
    }
    backend.install(&root_cert_path)
}
