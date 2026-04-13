use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = NatureResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum NatureResponse {
    NatureSet(NatureName),
    NatureDetails(Response<Nature>),
    Natures(Listed<Response<Nature>>),
    NoNatures,
    NatureRemoved(NatureName),
}
