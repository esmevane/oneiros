mod harness;
mod workflows;

pub(crate) use harness::*;
pub(crate) use workflows::*;

pub(crate) type TestResult = Result<(), Box<dyn core::error::Error>>;
