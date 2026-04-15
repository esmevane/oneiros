//! Toolsets — scoped capability sets for cognitive moments.
//!
//! The MCP server always exposes a small root layer. Activating a
//! toolset loads additional tools for a specific cognitive mode.
//! One toolset active at a time.

use core::fmt;

use serde::{Deserialize, Serialize};

/// A named set of tools that load together for a cognitive moment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Toolset {
    Lifecycle,
    Continuity,
    Vocabulary,
    Administer,
    Manage,
}

impl Toolset {
    /// All available toolsets.
    pub const ALL: &[Toolset] = &[
        Toolset::Lifecycle,
        Toolset::Continuity,
        Toolset::Vocabulary,
        Toolset::Administer,
        Toolset::Manage,
    ];

    /// Human-readable description of this toolset's purpose.
    pub fn description(&self) -> &'static str {
        match self {
            Toolset::Lifecycle => "Agent lifecycle — ceremonies, identity, sessions",
            Toolset::Continuity => "Cognitive work — thoughts, knowledge, connections, search",
            Toolset::Vocabulary => {
                "Shape the language — textures, levels, sensations, natures, personas, urges"
            }
            Toolset::Administer => "System administration — tenants, actors, brains, tickets",
            Toolset::Manage => "Timeline management — bookmarks, sharing, distribution",
        }
    }

    /// Tool names that belong to this toolset.
    pub fn tool_names(&self) -> &[&str] {
        match self {
            Toolset::Lifecycle => &[
                "wake-agent",
                "dream-agent",
                "introspect-agent",
                "reflect-agent",
                "sleep-agent",
                "emerge-agent",
                "recede-agent",
                "sense-content",
                "create-agent",
                "update-agent",
                "remove-agent",
            ],
            Toolset::Continuity => &[
                "add-cognition",
                "add-memory",
                "create-experience",
                "update-experience-description",
                "update-experience-sensation",
                "create-connection",
                "remove-connection",
                "search-query",
                "remove-storage",
            ],
            Toolset::Vocabulary => &[
                "set-level",
                "remove-level",
                "set-texture",
                "remove-texture",
                "set-sensation",
                "remove-sensation",
                "set-nature",
                "remove-nature",
                "set-persona",
                "remove-persona",
                "set-urge",
                "remove-urge",
            ],
            Toolset::Administer => &[
                "create-actor",
                "create-tenant",
                "create-brain",
                "create-ticket",
                "validate-ticket",
            ],
            Toolset::Manage => &[
                "create-bookmark",
                "switch-bookmark",
                "merge-bookmark",
                "share-bookmark",
                "follow-bookmark",
                "collect-bookmark",
                "unfollow-bookmark",
            ],
        }
    }

    /// Whether a tool name belongs to this toolset.
    pub fn contains(&self, tool_name: &str) -> bool {
        self.tool_names().contains(&tool_name)
    }
}

/// Tool names that are always available regardless of active toolset.
pub const ROOT_TOOLS: &[&str] = &[
    "activate-toolset",
    "deactivate-toolset",
    "get-pressure",
    "list-pressures",
];

/// Whether a tool name is in the root layer.
pub fn is_root_tool(name: &str) -> bool {
    ROOT_TOOLS.contains(&name)
}

/// Whether a tool name belongs to any toolset or the root layer.
pub fn is_cataloged(name: &str) -> bool {
    is_root_tool(name) || Toolset::ALL.iter().any(|ts| ts.contains(name))
}

impl fmt::Display for Toolset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Toolset::Lifecycle => write!(f, "lifecycle"),
            Toolset::Continuity => write!(f, "continuity"),
            Toolset::Vocabulary => write!(f, "vocabulary"),
            Toolset::Administer => write!(f, "administer"),
            Toolset::Manage => write!(f, "manage"),
        }
    }
}

impl core::str::FromStr for Toolset {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lifecycle" => Ok(Toolset::Lifecycle),
            "continuity" => Ok(Toolset::Continuity),
            "vocabulary" => Ok(Toolset::Vocabulary),
            "administer" => Ok(Toolset::Administer),
            "manage" => Ok(Toolset::Manage),
            other => Err(format!("unknown toolset: {other}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toolset_round_trips_through_str() {
        for toolset in Toolset::ALL {
            let s = toolset.to_string();
            let back: Toolset = s.parse().unwrap();
            assert_eq!(*toolset, back);
        }
    }

    #[test]
    fn toolset_round_trips_through_serde() {
        for toolset in Toolset::ALL {
            let json = serde_json::to_string(toolset).unwrap();
            let back: Toolset = serde_json::from_str(&json).unwrap();
            assert_eq!(*toolset, back);
        }
    }

    #[test]
    fn continuity_contains_add_cognition() {
        assert!(Toolset::Continuity.contains("add-cognition"));
        assert!(!Toolset::Continuity.contains("wake-agent"));
    }

    #[test]
    fn root_tools_are_not_in_any_toolset() {
        for name in ROOT_TOOLS {
            for toolset in Toolset::ALL {
                assert!(
                    !toolset.contains(name),
                    "root tool {name} should not be in {toolset}"
                );
            }
        }
    }

    #[test]
    fn every_toolset_tool_is_cataloged() {
        for toolset in Toolset::ALL {
            for name in toolset.tool_names() {
                assert!(is_cataloged(name), "{name} should be cataloged");
            }
        }
    }

    #[test]
    fn get_list_tools_are_not_cataloged() {
        assert!(!is_cataloged("get-agent"));
        assert!(!is_cataloged("list-agents"));
        assert!(!is_cataloged("get-cognition"));
        assert!(!is_cataloged("list-cognitions"));
        assert!(!is_cataloged("get-level"));
        assert!(!is_cataloged("list-levels"));
    }
}
