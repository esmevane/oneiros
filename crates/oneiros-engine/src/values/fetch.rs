use std::future::Future;
use std::time::Duration;

use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

fn default_interval() -> Duration {
    Duration::from_millis(16)
}

fn default_timeout() -> Duration {
    Duration::from_secs(2)
}

/// Configuration for eventually-consistent reads.
///
/// Carries the polling cadence and the maximum patience window for
/// reads that may need to wait for projections to catch up. Carried by
/// `Config` as the server-level default. Per-request overrides will
/// arrive when the request envelope is in place.
#[derive(Args, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Fetch {
    /// How often to poll while waiting for an eventually-consistent read.
    #[arg(long, global = true, default_value = "16ms", value_parser = humantime::parse_duration)]
    #[serde(default = "default_interval", with = "humantime_serde")]
    #[schemars(with = "String")]
    pub interval: Duration,
    /// Maximum time to wait before giving up.
    #[arg(long, global = true, default_value = "2s", value_parser = humantime::parse_duration)]
    #[serde(default = "default_timeout", with = "humantime_serde")]
    #[schemars(with = "String")]
    pub timeout: Duration,
}

impl Default for Fetch {
    fn default() -> Self {
        Self {
            interval: default_interval(),
            timeout: default_timeout(),
        }
    }
}

impl Fetch {
    /// Poll the closure until it yields a value or the patience window
    /// expires.
    ///
    /// Returns `Ok(Some(_))` when the closure produces a value within
    /// `timeout`, `Ok(None)` when the window expires without a value,
    /// and `Err(_)` if the closure errors.
    pub async fn eventual<T, E, F, Fut>(&self, f: F) -> Result<Option<T>, E>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<Option<T>, E>>,
    {
        tokio::time::timeout(self.timeout, async {
            loop {
                if let Some(value) = f().await? {
                    return Ok(Some(value));
                }
                tokio::time::sleep(self.interval).await;
            }
        })
        .await
        .unwrap_or(Ok(None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn default_values() {
        let fetch = Fetch::default();
        assert_eq!(fetch.interval, Duration::from_millis(16));
        assert_eq!(fetch.timeout, Duration::from_secs(2));
    }

    #[tokio::test(start_paused = true)]
    async fn eventual_returns_immediately_when_value_present() {
        let fetch = Fetch::default();
        let result: Result<Option<i32>, ()> = fetch.eventual(|| async { Ok(Some(42)) }).await;
        assert_eq!(result, Ok(Some(42)));
    }

    #[tokio::test(start_paused = true)]
    async fn eventual_returns_none_after_timeout_expires() {
        let fetch = Fetch {
            interval: Duration::from_millis(10),
            timeout: Duration::from_millis(100),
        };
        let result: Result<Option<i32>, ()> = fetch.eventual(|| async { Ok(None) }).await;
        assert_eq!(result, Ok(None));
    }

    #[tokio::test(start_paused = true)]
    async fn eventual_propagates_errors_from_closure() {
        #[derive(Debug, PartialEq)]
        struct MyErr;

        let fetch = Fetch::default();
        let result: Result<Option<i32>, MyErr> = fetch.eventual(|| async { Err(MyErr) }).await;
        assert_eq!(result, Err(MyErr));
    }

    #[tokio::test(start_paused = true)]
    async fn eventual_polls_until_value_appears() {
        let attempts = Arc::new(AtomicUsize::new(0));
        let attempts_clone = attempts.clone();

        let fetch = Fetch {
            interval: Duration::from_millis(10),
            timeout: Duration::from_secs(60),
        };

        let result: Result<Option<usize>, ()> = fetch
            .eventual(move || {
                let attempts = attempts_clone.clone();
                async move {
                    let n = attempts.fetch_add(1, Ordering::SeqCst);
                    if n < 3 { Ok(None) } else { Ok(Some(n)) }
                }
            })
            .await;

        assert_eq!(result, Ok(Some(3)));
        assert_eq!(attempts.load(Ordering::SeqCst), 4);
    }
}
