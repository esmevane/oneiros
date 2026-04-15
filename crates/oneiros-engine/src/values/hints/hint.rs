use serde::{Deserialize, Serialize};

use crate::*;

/// A navigational breadcrumb — tells a consuming agent what it can do
/// next from where it is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hint {
    pub level: HintLevel,
    pub action: String,
    pub intent: String,
}

impl Hint {
    pub fn suggest(action: impl Into<String>, intent: impl Into<String>) -> Self {
        Self {
            level: HintLevel::Suggest,
            action: action.into(),
            intent: intent.into(),
        }
    }

    pub fn inspect(action: impl Into<String>, intent: impl Into<String>) -> Self {
        Self {
            level: HintLevel::Inspect,
            action: action.into(),
            intent: intent.into(),
        }
    }

    pub fn follow_up(action: impl Into<String>, intent: impl Into<String>) -> Self {
        Self {
            level: HintLevel::FollowUp,
            action: action.into(),
            intent: intent.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn hint_constructors_set_level() {
        let s = Hint::suggest("action", "intent");
        assert_eq!(s.level, HintLevel::Suggest);

        let i = Hint::inspect("action", "intent");
        assert_eq!(i.level, HintLevel::Inspect);

        let f = Hint::follow_up("action", "intent");
        assert_eq!(f.level, HintLevel::FollowUp);
    }

    #[test]
    fn hint_serde_round_trip() {
        let hint = Hint::follow_up("reflect governor.process", "Pause on something significant");
        let json = serde_json::to_string(&hint).unwrap();
        let back: Hint = serde_json::from_str(&json).unwrap();
        assert_eq!(hint, back);
        assert!(json.contains("\"follow-up\""));
    }
}
