use std::time::Duration;

use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

fn default_fetch_interval_ms() -> u64 {
    16
}

fn default_fetch_timeout_ms() -> u64 {
    2000
}

/// Eventually-tolerant read policy.
///
/// `repo.get(key)` returns "is this here right now?". `repo.fetch(key,
/// policy)` polls that read until the answer arrives or `timeout`
/// expires — giving callers a coherent way to wait for projections to
/// catch up to a just-published event without barriers, acks, or
/// projection inlining.
///
/// Carried by `Config` as the server-wide default. Per-call sites can
/// construct their own (e.g. for shorter waits in tight loops, or
/// longer waits for slow imports) and override the default.
#[derive(Args, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FetchPolicy {
    /// How often to retry while waiting for the resource to appear.
    #[arg(long, global = true, default_value_t = default_fetch_interval_ms())]
    #[serde(default = "default_fetch_interval_ms")]
    pub fetch_interval_ms: u64,
    /// How long to wait before giving up and returning `Ok(None)`.
    #[arg(long, global = true, default_value_t = default_fetch_timeout_ms())]
    #[serde(default = "default_fetch_timeout_ms")]
    pub fetch_timeout_ms: u64,
}

impl Default for FetchPolicy {
    fn default() -> Self {
        Self {
            fetch_interval_ms: default_fetch_interval_ms(),
            fetch_timeout_ms: default_fetch_timeout_ms(),
        }
    }
}

impl FetchPolicy {
    /// A policy that does not wait — equivalent to a single `get`.
    pub fn immediate() -> Self {
        Self {
            fetch_interval_ms: 0,
            fetch_timeout_ms: 0,
        }
    }

    pub fn interval(&self) -> Duration {
        Duration::from_millis(self.fetch_interval_ms)
    }

    pub fn timeout(&self) -> Duration {
        Duration::from_millis(self.fetch_timeout_ms)
    }
}

/// Poll an async lookup until it resolves to `Some(_)` or the policy
/// times out. Errors short-circuit immediately.
///
/// This is the underlying mechanism behind `repo.fetch`. Each repo
/// exposes its own typed `fetch` that delegates here so callers stay
/// fluent: `AgentRepo::new(scope).fetch(name, &policy).await?`.
pub async fn fetch_eventually<T, F, Fut, E>(policy: &FetchPolicy, mut lookup: F) -> Result<Option<T>, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<Option<T>, E>>,
{
    let deadline = std::time::Instant::now() + policy.timeout();
    let interval = policy.interval();

    loop {
        match lookup().await? {
            Some(value) => return Ok(Some(value)),
            None => {
                if std::time::Instant::now() >= deadline {
                    return Ok(None);
                }
                tokio::time::sleep(interval).await;
            }
        }
    }
}
