use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = MemoryEventsType, display = "kebab-case")]
pub enum MemoryEvents {
    MemoryAdded(Memory),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_types_are_kebab_cased() {
        assert_eq!(&MemoryEventsType::MemoryAdded.to_string(), "memory-added");
    }
}
