use serde::{Deserialize, Serialize};

/// How strongly a hint recommends the action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HintLevel {
    /// A gentle suggestion — might be useful.
    Suggest,
    /// Worth looking at — inspect current state.
    Inspect,
    /// Recommended next step — something wants attention.
    FollowUp,
}

impl core::fmt::Display for HintLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            HintLevel::Suggest => write!(f, "suggest"),
            HintLevel::Inspect => write!(f, "inspect"),
            HintLevel::FollowUp => write!(f, "follow-up"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn hint_level_serializes_kebab_case() {
        let json = serde_json::to_string(&HintLevel::FollowUp).unwrap();
        assert_eq!(json, "\"follow-up\"");

        let json = serde_json::to_string(&HintLevel::Suggest).unwrap();
        assert_eq!(json, "\"suggest\"");

        let json = serde_json::to_string(&HintLevel::Inspect).unwrap();
        assert_eq!(json, "\"inspect\"");
    }

    #[test]
    fn hint_level_round_trips() {
        for level in [HintLevel::Suggest, HintLevel::Inspect, HintLevel::FollowUp] {
            let json = serde_json::to_string(&level).unwrap();
            let back: HintLevel = serde_json::from_str(&json).unwrap();
            assert_eq!(level, back);
        }
    }
}
