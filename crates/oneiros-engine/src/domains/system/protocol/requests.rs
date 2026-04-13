use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct InitSystem {
    #[arg(long, short)]
    #[builder(into)]
    pub(crate) name: Option<String>,
    #[arg(long, short)]
    #[builder(default)]
    pub(crate) yes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = SystemRequestType, display = "kebab-case")]
pub(crate) enum SystemRequest {
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
