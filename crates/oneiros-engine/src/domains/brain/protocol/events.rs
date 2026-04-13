use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = BrainEventsType, display = "kebab-case")]
pub(crate) enum BrainEvents {
    BrainCreated(Brain),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_types_are_kebab_cased() {
        assert_eq!(&BrainEventsType::BrainCreated.to_string(), "brain-created");
    }
}
