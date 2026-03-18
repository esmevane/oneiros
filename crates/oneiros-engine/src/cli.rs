//! CLI command definitions — clap-derived command tree.
//!
//! Each domain contributes subcommands. The CLI delegates to domain
//! services via the project or system context, printing results as JSON.

use clap::Subcommand;

use crate::contexts::ProjectContext;
use crate::contexts::SystemContext;

/// Top-level CLI commands for the engine.
#[derive(Debug, Subcommand)]
pub enum Commands {
    // ── Vocabulary ───────────────────────────────────────────────
    #[command(subcommand)]
    Level(VocabCommands),
    #[command(subcommand)]
    Texture(VocabCommands),
    #[command(subcommand)]
    Sensation(VocabCommands),
    #[command(subcommand)]
    Nature(VocabCommands),
    #[command(subcommand)]
    Persona(VocabCommands),
    #[command(subcommand)]
    Urge(VocabCommands),

    // ── Agent ────────────────────────────────────────────────────
    #[command(subcommand)]
    Agent(AgentCommands),

    // ── Cognition ────────────────────────────────────────────────
    #[command(subcommand)]
    Cognition(CognitionCommands),

    // ── Memory ───────────────────────────────────────────────────
    #[command(subcommand)]
    Memory(MemoryCommands),

    // ── Experience ───────────────────────────────────────────────
    #[command(subcommand)]
    Experience(ExperienceCommands),

    // ── Connection ───────────────────────────────────────────────
    #[command(subcommand)]
    Connection(ConnectionCommands),

    // ── Lifecycle ────────────────────────────────────────────────
    Dream {
        agent: String,
    },
    Introspect {
        agent: String,
    },
    Reflect {
        agent: String,
    },
    Sense {
        agent: String,
        content: String,
    },
    Sleep {
        agent: String,
    },

    // ── Search ───────────────────────────────────────────────────
    Search {
        query: String,
        #[arg(long)]
        agent: Option<String>,
    },

    // ── Pressure ─────────────────────────────────────────────────
    #[command(subcommand)]
    Pressure(PressureCommands),
}

#[derive(Debug, Subcommand)]
pub enum VocabCommands {
    Set {
        name: String,
        #[arg(long, default_value = "")]
        description: String,
        #[arg(long, default_value = "")]
        prompt: String,
    },
    Get {
        name: String,
    },
    List,
    Remove {
        name: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum AgentCommands {
    Create {
        name: String,
        #[arg(long)]
        persona: String,
        #[arg(long, default_value = "")]
        description: String,
        #[arg(long, default_value = "")]
        prompt: String,
    },
    Get {
        name: String,
    },
    List,
    Update {
        name: String,
        #[arg(long)]
        persona: String,
        #[arg(long, default_value = "")]
        description: String,
        #[arg(long, default_value = "")]
        prompt: String,
    },
    Remove {
        name: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum CognitionCommands {
    Add {
        agent: String,
        texture: String,
        content: String,
    },
    Get {
        id: String,
    },
    List {
        #[arg(long)]
        agent: Option<String>,
        #[arg(long)]
        texture: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
pub enum MemoryCommands {
    Add {
        agent: String,
        level: String,
        content: String,
    },
    Get {
        id: String,
    },
    List {
        #[arg(long)]
        agent: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
pub enum ExperienceCommands {
    Create {
        agent: String,
        sensation: String,
        description: String,
    },
    Get {
        id: String,
    },
    List {
        #[arg(long)]
        agent: Option<String>,
    },
    UpdateDescription {
        id: String,
        description: String,
    },
    UpdateSensation {
        id: String,
        sensation: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum ConnectionCommands {
    Create {
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        nature: String,
        description: String,
    },
    Get {
        id: String,
    },
    List {
        #[arg(long)]
        entity: Option<String>,
    },
    Remove {
        id: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum PressureCommands {
    Get { agent: String },
    List,
}

/// System-scoped CLI commands.
#[derive(Debug, Subcommand)]
pub enum SystemCommands {
    #[command(subcommand)]
    Tenant(TenantCommands),
    #[command(subcommand)]
    Actor(ActorCommands),
    #[command(subcommand)]
    Brain(BrainCommands),
    #[command(subcommand)]
    Ticket(TicketCommands),
}

#[derive(Debug, Subcommand)]
pub enum TenantCommands {
    Create { name: String },
    Get { id: String },
    List,
}

#[derive(Debug, Subcommand)]
pub enum ActorCommands {
    Create {
        #[arg(long)]
        tenant_id: String,
        name: String,
    },
    Get {
        id: String,
    },
    List,
}

#[derive(Debug, Subcommand)]
pub enum BrainCommands {
    Create { name: String },
    Get { name: String },
    List,
}

#[derive(Debug, Subcommand)]
pub enum TicketCommands {
    Issue {
        #[arg(long)]
        actor_id: String,
        #[arg(long)]
        brain_name: String,
    },
    Validate {
        id: String,
    },
    List,
}

/// Execute a project-scoped command against the given context.
///
/// Returns the JSON-serialized result as a string.
pub fn execute(
    ctx: &ProjectContext,
    command: Commands,
) -> Result<String, Box<dyn std::error::Error>> {
    match command {
        // ── Vocabulary domains ───────────────────────────────────
        Commands::Level(cmd) => execute_vocab(ctx, cmd, VocabDomain::Level),
        Commands::Texture(cmd) => execute_vocab(ctx, cmd, VocabDomain::Texture),
        Commands::Sensation(cmd) => execute_vocab(ctx, cmd, VocabDomain::Sensation),
        Commands::Nature(cmd) => execute_vocab(ctx, cmd, VocabDomain::Nature),
        Commands::Persona(cmd) => execute_vocab(ctx, cmd, VocabDomain::Persona),
        Commands::Urge(cmd) => execute_vocab(ctx, cmd, VocabDomain::Urge),

        // ── Agent ────────────────────────────────────────────────
        Commands::Agent(cmd) => {
            use crate::domains::agent::service::AgentService;
            let result = match cmd {
                AgentCommands::Create {
                    name,
                    persona,
                    description,
                    prompt,
                } => serde_json::to_string_pretty(&AgentService::create(
                    ctx,
                    name,
                    persona,
                    description,
                    prompt,
                )?)?,
                AgentCommands::Get { name } => {
                    serde_json::to_string_pretty(&AgentService::get(ctx, &name)?)?
                }
                AgentCommands::List => serde_json::to_string_pretty(&AgentService::list(ctx)?)?,
                AgentCommands::Update {
                    name,
                    persona,
                    description,
                    prompt,
                } => serde_json::to_string_pretty(&AgentService::update(
                    ctx,
                    name,
                    persona,
                    description,
                    prompt,
                )?)?,
                AgentCommands::Remove { name } => {
                    serde_json::to_string_pretty(&AgentService::remove(ctx, &name)?)?
                }
            };
            Ok(result)
        }

        // ── Cognition ────────────────────────────────────────────
        Commands::Cognition(cmd) => {
            use crate::domains::cognition::service::CognitionService;
            let result = match cmd {
                CognitionCommands::Add {
                    agent,
                    texture,
                    content,
                } => serde_json::to_string_pretty(&CognitionService::add(
                    ctx, agent, texture, content,
                )?)?,
                CognitionCommands::Get { id } => {
                    serde_json::to_string_pretty(&CognitionService::get(ctx, &id)?)?
                }
                CognitionCommands::List { agent, texture } => serde_json::to_string_pretty(
                    &CognitionService::list(ctx, agent.as_deref(), texture.as_deref())?,
                )?,
            };
            Ok(result)
        }

        // ── Memory ───────────────────────────────────────────────
        Commands::Memory(cmd) => {
            use crate::domains::memory::service::MemoryService;
            let result = match cmd {
                MemoryCommands::Add {
                    agent,
                    level,
                    content,
                } => {
                    serde_json::to_string_pretty(&MemoryService::add(ctx, agent, level, content)?)?
                }
                MemoryCommands::Get { id } => {
                    serde_json::to_string_pretty(&MemoryService::get(ctx, &id)?)?
                }
                MemoryCommands::List { agent } => {
                    serde_json::to_string_pretty(&MemoryService::list(ctx, agent.as_deref())?)?
                }
            };
            Ok(result)
        }

        // ── Experience ───────────────────────────────────────────
        Commands::Experience(cmd) => {
            use crate::domains::experience::service::ExperienceService;
            let result = match cmd {
                ExperienceCommands::Create {
                    agent,
                    sensation,
                    description,
                } => serde_json::to_string_pretty(&ExperienceService::create(
                    ctx,
                    agent,
                    sensation,
                    description,
                )?)?,
                ExperienceCommands::Get { id } => {
                    serde_json::to_string_pretty(&ExperienceService::get(ctx, &id)?)?
                }
                ExperienceCommands::List { agent } => {
                    serde_json::to_string_pretty(&ExperienceService::list(ctx, agent.as_deref())?)?
                }
                ExperienceCommands::UpdateDescription { id, description } => {
                    serde_json::to_string_pretty(&ExperienceService::update_description(
                        ctx,
                        &id,
                        description,
                    )?)?
                }
                ExperienceCommands::UpdateSensation { id, sensation } => {
                    serde_json::to_string_pretty(&ExperienceService::update_sensation(
                        ctx, &id, sensation,
                    )?)?
                }
            };
            Ok(result)
        }

        // ── Connection ───────────────────────────────────────────
        Commands::Connection(cmd) => {
            use crate::domains::connection::service::ConnectionService;
            let result = match cmd {
                ConnectionCommands::Create {
                    from,
                    to,
                    nature,
                    description,
                } => serde_json::to_string_pretty(&ConnectionService::create(
                    ctx,
                    from,
                    to,
                    nature,
                    description,
                )?)?,
                ConnectionCommands::Get { id } => {
                    serde_json::to_string_pretty(&ConnectionService::get(ctx, &id)?)?
                }
                ConnectionCommands::List { entity } => {
                    serde_json::to_string_pretty(&ConnectionService::list(ctx, entity.as_deref())?)?
                }
                ConnectionCommands::Remove { id } => {
                    serde_json::to_string_pretty(&ConnectionService::remove(ctx, &id)?)?
                }
            };
            Ok(result)
        }

        // ── Lifecycle ────────────────────────────────────────────
        Commands::Dream { agent } => {
            use crate::domains::lifecycle::service::LifecycleService;
            Ok(serde_json::to_string_pretty(&LifecycleService::dream(
                ctx, &agent,
            )?)?)
        }
        Commands::Introspect { agent } => {
            use crate::domains::lifecycle::service::LifecycleService;
            Ok(serde_json::to_string_pretty(
                &LifecycleService::introspect(ctx, &agent)?,
            )?)
        }
        Commands::Reflect { agent } => {
            use crate::domains::lifecycle::service::LifecycleService;
            Ok(serde_json::to_string_pretty(&LifecycleService::reflect(
                ctx, &agent,
            )?)?)
        }
        Commands::Sense { agent, content } => {
            use crate::domains::lifecycle::service::LifecycleService;
            Ok(serde_json::to_string_pretty(&LifecycleService::sense(
                ctx, &agent, &content,
            )?)?)
        }
        Commands::Sleep { agent } => {
            use crate::domains::lifecycle::service::LifecycleService;
            Ok(serde_json::to_string_pretty(&LifecycleService::sleep(
                ctx, &agent,
            )?)?)
        }

        // ── Search ───────────────────────────────────────────────
        Commands::Search { query, agent } => {
            use crate::domains::search::service::SearchService;
            Ok(serde_json::to_string_pretty(&SearchService::search(
                ctx,
                &query,
                agent.as_deref(),
            )?)?)
        }

        // ── Pressure ─────────────────────────────────────────────
        Commands::Pressure(cmd) => {
            use crate::domains::pressure::service::PressureService;
            let result = match cmd {
                PressureCommands::Get { agent } => {
                    serde_json::to_string_pretty(&PressureService::get(ctx, &agent)?)?
                }
                PressureCommands::List => {
                    serde_json::to_string_pretty(&PressureService::list(ctx)?)?
                }
            };
            Ok(result)
        }
    }
}

/// Execute a system-scoped command.
pub fn execute_system(
    ctx: &SystemContext,
    command: SystemCommands,
) -> Result<String, Box<dyn std::error::Error>> {
    match command {
        SystemCommands::Tenant(cmd) => {
            use crate::domains::tenant::service::TenantService;
            let result = match cmd {
                TenantCommands::Create { name } => {
                    serde_json::to_string_pretty(&TenantService::create(ctx, name)?)?
                }
                TenantCommands::Get { id } => {
                    serde_json::to_string_pretty(&TenantService::get(ctx, &id)?)?
                }
                TenantCommands::List => serde_json::to_string_pretty(&TenantService::list(ctx)?)?,
            };
            Ok(result)
        }
        SystemCommands::Actor(cmd) => {
            use crate::domains::actor::service::ActorService;
            let result = match cmd {
                ActorCommands::Create { tenant_id, name } => {
                    serde_json::to_string_pretty(&ActorService::create(ctx, tenant_id, name)?)?
                }
                ActorCommands::Get { id } => {
                    serde_json::to_string_pretty(&ActorService::get(ctx, &id)?)?
                }
                ActorCommands::List => serde_json::to_string_pretty(&ActorService::list(ctx)?)?,
            };
            Ok(result)
        }
        SystemCommands::Brain(cmd) => {
            use crate::domains::brain::service::BrainService;
            let result = match cmd {
                BrainCommands::Create { name } => {
                    serde_json::to_string_pretty(&BrainService::create(ctx, name)?)?
                }
                BrainCommands::Get { name } => {
                    serde_json::to_string_pretty(&BrainService::get(ctx, &name)?)?
                }
                BrainCommands::List => serde_json::to_string_pretty(&BrainService::list(ctx)?)?,
            };
            Ok(result)
        }
        SystemCommands::Ticket(cmd) => {
            use crate::domains::ticket::service::TicketService;
            let result = match cmd {
                TicketCommands::Issue {
                    actor_id,
                    brain_name,
                } => serde_json::to_string_pretty(&TicketService::create(
                    ctx, actor_id, brain_name,
                )?)?,
                TicketCommands::Validate { id } => {
                    serde_json::to_string_pretty(&TicketService::validate(ctx, &id)?)?
                }
                TicketCommands::List => serde_json::to_string_pretty(&TicketService::list(ctx)?)?,
            };
            Ok(result)
        }
    }
}

// ── Vocabulary dispatch helper ───────────────────────────────────

enum VocabDomain {
    Level,
    Texture,
    Sensation,
    Nature,
    Persona,
    Urge,
}

fn execute_vocab(
    ctx: &ProjectContext,
    cmd: VocabCommands,
    domain: VocabDomain,
) -> Result<String, Box<dyn std::error::Error>> {
    match domain {
        VocabDomain::Level => {
            use crate::domains::level::{model::Level, service::LevelService};
            match cmd {
                VocabCommands::Set {
                    name,
                    description,
                    prompt,
                } => Ok(serde_json::to_string_pretty(&LevelService::set(
                    ctx,
                    Level {
                        name,
                        description,
                        prompt,
                    },
                )?)?),
                VocabCommands::Get { name } => Ok(serde_json::to_string_pretty(
                    &LevelService::get(ctx, &name)?,
                )?),
                VocabCommands::List => Ok(serde_json::to_string_pretty(&LevelService::list(ctx)?)?),
                VocabCommands::Remove { name } => Ok(serde_json::to_string_pretty(
                    &LevelService::remove(ctx, &name)?,
                )?),
            }
        }
        VocabDomain::Texture => {
            use crate::domains::texture::{model::Texture, service::TextureService};
            match cmd {
                VocabCommands::Set {
                    name,
                    description,
                    prompt,
                } => Ok(serde_json::to_string_pretty(&TextureService::set(
                    ctx,
                    Texture {
                        name,
                        description,
                        prompt,
                    },
                )?)?),
                VocabCommands::Get { name } => Ok(serde_json::to_string_pretty(
                    &TextureService::get(ctx, &name)?,
                )?),
                VocabCommands::List => {
                    Ok(serde_json::to_string_pretty(&TextureService::list(ctx)?)?)
                }
                VocabCommands::Remove { name } => Ok(serde_json::to_string_pretty(
                    &TextureService::remove(ctx, &name)?,
                )?),
            }
        }
        VocabDomain::Sensation => {
            use crate::domains::sensation::{model::Sensation, service::SensationService};
            match cmd {
                VocabCommands::Set {
                    name,
                    description,
                    prompt,
                } => Ok(serde_json::to_string_pretty(&SensationService::set(
                    ctx,
                    Sensation {
                        name,
                        description,
                        prompt,
                    },
                )?)?),
                VocabCommands::Get { name } => Ok(serde_json::to_string_pretty(
                    &SensationService::get(ctx, &name)?,
                )?),
                VocabCommands::List => {
                    Ok(serde_json::to_string_pretty(&SensationService::list(ctx)?)?)
                }
                VocabCommands::Remove { name } => Ok(serde_json::to_string_pretty(
                    &SensationService::remove(ctx, &name)?,
                )?),
            }
        }
        VocabDomain::Nature => {
            use crate::domains::nature::{model::Nature, service::NatureService};
            match cmd {
                VocabCommands::Set {
                    name,
                    description,
                    prompt,
                } => Ok(serde_json::to_string_pretty(&NatureService::set(
                    ctx,
                    Nature {
                        name,
                        description,
                        prompt,
                    },
                )?)?),
                VocabCommands::Get { name } => Ok(serde_json::to_string_pretty(
                    &NatureService::get(ctx, &name)?,
                )?),
                VocabCommands::List => {
                    Ok(serde_json::to_string_pretty(&NatureService::list(ctx)?)?)
                }
                VocabCommands::Remove { name } => Ok(serde_json::to_string_pretty(
                    &NatureService::remove(ctx, &name)?,
                )?),
            }
        }
        VocabDomain::Persona => {
            use crate::domains::persona::{model::Persona, service::PersonaService};
            match cmd {
                VocabCommands::Set {
                    name,
                    description,
                    prompt,
                } => Ok(serde_json::to_string_pretty(&PersonaService::set(
                    ctx,
                    Persona {
                        name,
                        description,
                        prompt,
                    },
                )?)?),
                VocabCommands::Get { name } => Ok(serde_json::to_string_pretty(
                    &PersonaService::get(ctx, &name)?,
                )?),
                VocabCommands::List => {
                    Ok(serde_json::to_string_pretty(&PersonaService::list(ctx)?)?)
                }
                VocabCommands::Remove { name } => Ok(serde_json::to_string_pretty(
                    &PersonaService::remove(ctx, &name)?,
                )?),
            }
        }
        VocabDomain::Urge => {
            use crate::domains::urge::{model::Urge, service::UrgeService};
            match cmd {
                VocabCommands::Set {
                    name,
                    description,
                    prompt,
                } => Ok(serde_json::to_string_pretty(&UrgeService::set(
                    ctx,
                    Urge {
                        name,
                        description,
                        prompt,
                    },
                )?)?),
                VocabCommands::Get { name } => Ok(serde_json::to_string_pretty(
                    &UrgeService::get(ctx, &name)?,
                )?),
                VocabCommands::List => Ok(serde_json::to_string_pretty(&UrgeService::list(ctx)?)?),
                VocabCommands::Remove { name } => Ok(serde_json::to_string_pretty(
                    &UrgeService::remove(ctx, &name)?,
                )?),
            }
        }
    }
}
