use clap::Args;
use oneiros_outcomes::{Outcome, Outcomes};
use oneiros_trust::{TrustMode, TrustProvider};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum TrustInitError {
    #[error("Trust error: {0}")]
    Trust(#[from] oneiros_trust::TrustError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
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
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<TrustInitOutcomes>, TrustInitError> {
        let mut outcomes = Outcomes::new();

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

        // Build a TrustConfig with the selected mode
        let mut trust_config = config.trust.clone();
        trust_config.mode = mode;

        // Initialize the trust provider (creates CA, issues certs if needed)
        let mut provider =
            TrustProvider::init(&trust_config, context.data_dir(), hostname)?;

        let health = provider.health();

        if health.ca_status == oneiros_trust::CaStatus::Valid {
            outcomes.emit(TrustInitOutcomes::CaCreated);
        }

        if health.leaf_status == oneiros_trust::LeafStatus::Valid {
            outcomes.emit(TrustInitOutcomes::LeafIssued(hostname.to_string()));
        }

        // Trust store installation — only if explicitly requested or interactively approved
        let should_install = self.install || (!self.yes && self.prompt_install(context));

        if should_install {
            match provider.install_trust_store() {
                Ok(true) => outcomes.emit(TrustInitOutcomes::TrustStoreInstalled),
                Ok(false) => outcomes.emit(TrustInitOutcomes::TrustStoreSkipped),
                Err(err) => {
                    outcomes.emit(TrustInitOutcomes::TrustStoreInstallFailed(err.to_string()))
                }
            }
        } else {
            outcomes.emit(TrustInitOutcomes::TrustStoreSkipped);
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
