mod harness;

pub(crate) use harness::*;

pub(crate) type TestResult = Result<(), Box<dyn core::error::Error>>;
