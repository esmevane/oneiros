mod error;
mod log_config;
mod outcomes;
mod output_format;

use clap::Parser;
use clap::Subcommand;
use oneiros_outcomes::Outcomes;

pub(crate) use error::*;
pub(crate) use log_config::LogConfig;
pub(crate) use outcomes::CliOutcomes;
pub(crate) use output_format::OutputFormat;

use crate::*;

#[derive(Clone, Parser)]
#[command(version)]
pub(crate) struct Cli {
    #[command(flatten)]
    pub(crate) log: LogConfig,
    /// Output format: prompt (default), quiet, or json.
    #[arg(long, short, default_value = "prompt", global = true)]
    pub(crate) output: OutputFormat,
    #[command(subcommand)]
    pub(crate) command: Command,
}

impl Cli {
    pub(crate) fn report(&self, outcomes: &Outcomes<CliOutcomes>) {
        for outcome in outcomes {
            self.output.structured_output(outcome);
        }
    }

    pub(crate) async fn run(&self) -> Result<Outcomes<CliOutcomes>, CliError> {
        let context = Context::init()?;

        Ok(match &self.command {
            Command::Activity(activity) => activity.run(&context).await?.map_into(),
            Command::Agent(agent) => agent.run(&context).await?.map_into(),
            Command::Cognition(cognition) => cognition.run(&context).await?.map_into(),
            Command::Connection(connection) => connection.run(&context).await?.map_into(),
            Command::Doctor(doctor) => doctor.run(&context).await?.map_into(),
            Command::Dream(dream) => dream.run(&context).await?.map_into(),
            Command::Emerge(emerge) => emerge.run(&context).await?.map_into(),
            Command::Experience(experience) => experience.run(&context).await?.map_into(),
            Command::Event(event) => event.run(&context).await?.map_into(),
            Command::Sensation(sensation) => sensation.run(&context).await?.map_into(),
            Command::Guidebook(guidebook) => guidebook.run(&context).await?.map_into(),
            Command::Introspect(introspect) => introspect.run(&context).await?.map_into(),
            Command::Level(level) => level.run(&context).await?.map_into(),
            Command::Memory(memory) => memory.run(&context).await?.map_into(),
            Command::Nature(nature) => nature.run(&context).await?.map_into(),
            Command::Persona(persona) => persona.run(&context).await?.map_into(),
            Command::Recede(recede) => recede.run(&context).await?.map_into(),
            Command::Reflect(reflect) => reflect.run(&context).await?.map_into(),
            Command::Search(search) => search.run(&context).await?.map_into(),
            Command::Seed(seed) => seed.run(&context).await?.map_into(),
            Command::Sense(sense) => sense.run(&context).await?.map_into(),
            Command::Skill(skill) => skill.run(&context).await?.map_into(),
            Command::Sleep(sleep) => sleep.run(&context).await?.map_into(),
            Command::Status(status) => status.run(&context).await?.map_into(),
            Command::Storage(storage) => storage.run(&context).await?.map_into(),
            Command::System(system) => system.run(&context).await?.map_into(),
            Command::Service(service) => service.run(&context).await?.map_into(),
            Command::Project(project) => project.run(&context).await?.map_into(),
            Command::Texture(texture) => texture.run(&context).await?.map_into(),
            Command::Wake(wake) => wake.run(&context).await?.map_into(),
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
    /// Wake an agent — start a session with dreaming.
    Wake(WakeOp),
}
