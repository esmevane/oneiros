mod error;
mod log_config;
mod outcomes;
mod output_format;
mod preflight;

use clap::Parser;
use clap::Subcommand;
use oneiros_outcomes::Outcomes;

pub(crate) use error::*;
pub(crate) use log_config::LogConfig;
pub(crate) use outcomes::CliOutcomes;
pub(crate) use output_format::OutputFormat;
pub(crate) use preflight::Preflight;

use oneiros_model::PressureSummary;

use crate::*;

/// The result of a CLI command: outcomes for display and ambient pressure summaries.
pub struct CliResult {
    pub outcomes: Outcomes<CliOutcomes>,
    pub summaries: Vec<PressureSummary>,
}

#[derive(Clone, Parser)]
#[command(version)]
pub struct Cli {
    #[command(flatten)]
    pub(crate) log: LogConfig,
    /// Output format: prompt (default), quiet, or json.
    #[arg(long, short, default_value = "prompt", global = true)]
    pub(crate) output: OutputFormat,
    #[command(subcommand)]
    pub(crate) command: Command,
}

impl Cli {
    pub(crate) fn report(&self, result: &CliResult) {
        for outcome in &result.outcomes {
            self.output.structured_output(outcome);
        }

        if !result.summaries.is_empty() {
            let pressures =
                oneiros_model::RelevantPressures::from_summaries(result.summaries.clone());
            eprintln!("{}", pressures.compact());
        }
    }

    pub(crate) async fn run(&self) -> Result<CliResult, CliError> {
        let context = Context::init()?;

        let (outcomes, summaries) = match &self.command {
            Command::Activity(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Agent(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Cognition(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Connection(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Doctor(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Dream(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Emerge(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Experience(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Event(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Sensation(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Guidebook(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Introspect(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Level(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Memory(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Nature(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Persona(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Pressure(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Recede(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Reflect(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Search(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Seed(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Sense(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Skill(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Sleep(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Status(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Storage(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::System(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Service(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Project(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Texture(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Urge(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
            Command::Wake(cmd) => {
                let (o, s) = cmd.run(&context).await?;
                (o.map_into(), s)
            }
        };

        Ok(CliResult {
            outcomes,
            summaries,
        })
    }
}

#[derive(Clone, Subcommand)]
pub(crate) enum Command {
    /// Monitor agent activity across cognitive domains.
    Activity(ActivityOps),
    /// Manage agents (named participants in a brain's cognition).
    Agent(AgentOps),
    /// Manage cognitions (thoughts logged by agents).
    Cognition(CognitionOps),
    /// Manage connections (edges between linked entities).
    Connection(ConnectionOps),
    /// Check the health of the local oneiros host and the current project.
    Doctor(DoctorOp),
    /// Compose an agent's full context into a dream prompt.
    Dream(DreamOp),
    /// Bring a new agent into existence.
    Emerge(EmergeOp),
    /// Manage experiences (descriptive edges connecting cognitive records).
    Experience(ExperienceOps),
    /// Inspect event log entries for a given brain
    Event(EventOps),
    /// Manage sensations (categories for experience relationships).
    Sensation(SensationOps),
    /// Show the cognitive guidebook for an agent.
    Guidebook(GuidebookOp),
    /// Summarize a session before context compaction.
    Introspect(IntrospectOp),
    /// Manage levels (memory retention tiers).
    Level(LevelOps),
    /// Manage memories (consolidated knowledge records).
    Memory(MemoryOps),
    /// Manage natures (categories for connection edges).
    Nature(NatureOps),
    /// Manage personas (named agent roles).
    Persona(PersonaOps),
    /// Show pressure readings for an agent.
    Pressure(PressureOp),
    /// Retire an agent from active service.
    Recede(RecedeOp),
    /// Reflect on a significant event during a session.
    Reflect(ReflectOp),
    /// Search the cognitive stream using full-text search.
    Search(SearchOp),
    /// Apply predefined seed data (textures, levels, personas, sensations, natures).
    Seed(SeedOps),
    /// Sense an observation — interpret an external event through an agent's cognitive lens.
    Sense(SenseOp),
    /// Manage the oneiros skill plugin.
    Skill(SkillOps),
    /// Put an agent to sleep — end a session with introspection.
    Sleep(SleepOp),
    /// Show a full cognitive status dashboard for an agent.
    Status(StatusOp),
    /// Manage content-addressable blob storage.
    Storage(StorageOps),
    /// Project-level commands (init, etc.).
    Project(ProjectOps),
    /// Manage the oneiros service (run, status).
    Service(ServiceOps),
    /// System-level commands for the local oneiros host (init, status, etc.).
    System(SystemOps),
    /// Manage textures (cognitive categories for agent thoughts).
    Texture(TextureOps),
    /// Manage urges (motivational drives for agent behavior).
    Urge(UrgeOps),
    /// Wake an agent — start a session with dreaming.
    Wake(WakeOp),
}
