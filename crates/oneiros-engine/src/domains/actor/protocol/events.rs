use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = ActorEventsType, display = "kebab-case")]
pub enum ActorEvents {
    ActorCreated(Actor),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_types_are_kebab_cased() {
        assert_eq!(&ActorEventsType::ActorCreated.to_string(), "actor-created");
    }
}
