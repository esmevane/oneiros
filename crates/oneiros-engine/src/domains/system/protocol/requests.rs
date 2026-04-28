use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum InitSystem {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long, short)]
            #[builder(into)]
            pub name: Option<String>,
            #[arg(long, short)]
            #[serde(default)]
            #[builder(default)]
            pub yes: bool,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = SystemRequestType, display = "kebab-case")]
pub enum SystemRequest {
    InitSystem(InitSystem),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [(SystemRequestType::InitSystem, "init-system")];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
