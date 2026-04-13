use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::*;

/// A named slice of the MCP tool surface.
///
/// The server always exposes its root layer (dashboard resources,
/// toolset management tools). The active Toolset controls which
/// additional tools appear alongside the root at any given moment.
///
/// Toolsets are organized by cognitive moment, not by domain. A single
/// toolset may draw tools from multiple domains. A single domain's
/// tools may appear in multiple toolsets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Toolset {
    /// Session lifecycle — wake, dream, introspect, sleep, emerge, recede
    Lifecycle,
    /// Mid-work capture — add cognition, reflect, sense, connect
    Capture,
    /// Consolidation — memory add, experience create, connection create, search
    Garden,
    /// Setup and evolution — system init, project init, seed, agent CRUD, vocabulary
    Admin,
    /// Sharing — bookmark share/follow/collect/merge, export, import
    Distribute,
}

impl Toolset {
    /// All available toolsets.
    pub fn all() -> &'static [Toolset] {
        &[
            Toolset::Lifecycle,
            Toolset::Capture,
            Toolset::Garden,
            Toolset::Admin,
            Toolset::Distribute,
        ]
    }

    /// Tool definitions for this toolset.
    ///
    /// Each toolset composes from existing domain tool catalogs.
    /// Tools may appear in multiple toolsets — overlap is intentional.
    pub fn defs(&self) -> Vec<ToolDef> {
        match self {
            Toolset::Lifecycle => [
                ContinuityTools.defs(),
                BookmarkTools.defs(),
            ]
            .into_iter()
            .flatten()
            .collect(),

            Toolset::Capture => [
                CognitionTools.defs(),
                ConnectionTools.defs(),
                SearchTools.defs(),
            ]
            .into_iter()
            .flatten()
            .collect(),

            Toolset::Garden => [
                MemoryTools.defs(),
                ExperienceTools.defs(),
                ConnectionTools.defs(),
                SearchTools.defs(),
            ]
            .into_iter()
            .flatten()
            .collect(),

            Toolset::Admin => [
                BrainTools.defs(),
                AgentTools.defs(),
                PersonaTools.defs(),
                TextureTools.defs(),
                SensationTools.defs(),
                NatureTools.defs(),
                LevelTools.defs(),
                UrgeTools.defs(),
                ActorTools.defs(),
                TenantTools.defs(),
                TicketTools.defs(),
                StorageTools.defs(),
            ]
            .into_iter()
            .flatten()
            .collect(),

            Toolset::Distribute => [
                BookmarkTools.defs(),
            ]
            .into_iter()
            .flatten()
            .collect(),
        }
    }

    /// Short description of this toolset for agent-facing prompts.
    pub fn description(&self) -> &'static str {
        match self {
            Toolset::Lifecycle => "Session lifecycle — wake, dream, introspect, sleep, emerge, recede",
            Toolset::Capture => "Mid-work capture — record thoughts, draw connections, search",
            Toolset::Garden => "Consolidation — promote memories, name experiences, connect, search",
            Toolset::Admin => "Setup and evolution — brains, agents, vocabulary, storage",
            Toolset::Distribute => "Sharing — bookmarks, export, import",
        }
    }

    /// Number of tools in this toolset.
    pub fn tool_count(&self) -> usize {
        self.defs().len()
    }
}

impl fmt::Display for Toolset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Toolset::Lifecycle => write!(f, "lifecycle"),
            Toolset::Capture => write!(f, "capture"),
            Toolset::Garden => write!(f, "garden"),
            Toolset::Admin => write!(f, "admin"),
            Toolset::Distribute => write!(f, "distribute"),
        }
    }
}

impl FromStr for Toolset {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lifecycle" => Ok(Toolset::Lifecycle),
            "capture" => Ok(Toolset::Capture),
            "garden" => Ok(Toolset::Garden),
            "admin" => Ok(Toolset::Admin),
            "distribute" => Ok(Toolset::Distribute),
            other => Err(format!(
                "Unknown toolset: '{other}'. Available: lifecycle, capture, garden, admin, distribute"
            )),
        }
    }
}

/// Request to activate a toolset — changes which tools are available.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ActivateToolset {
    /// The toolset to load. Options: lifecycle, capture, garden, admin, distribute.
    pub toolset: String,
}

/// Request to deactivate the current toolset — returns to root only.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct DeactivateToolset {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toolset_round_trips_through_string() {
        for toolset in Toolset::all() {
            let s = toolset.to_string();
            let parsed: Toolset = s.parse().unwrap();
            assert_eq!(&parsed, toolset);
        }
    }

    #[test]
    fn unknown_toolset_gives_helpful_error() {
        let err = "bogus".parse::<Toolset>().unwrap_err();
        assert!(err.contains("Unknown toolset"));
        assert!(err.contains("lifecycle"));
    }

    #[test]
    fn toolset_serde_round_trip() {
        for toolset in Toolset::all() {
            let json = serde_json::to_string(toolset).unwrap();
            let parsed: Toolset = serde_json::from_str(&json).unwrap();
            assert_eq!(&parsed, toolset);
        }
    }
}
