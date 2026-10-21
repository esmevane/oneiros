//! Contention workflow — concurrent requests against one bookmark.
//!
//! Several agents (or one impatient client) writing at once must each get
//! a successful response, and each write must land exactly once. Issue #323:
//! the pool used to answer contention with `DbError::CheckedOut`, failing
//! requests whose writes had already been committed.

use crate::tests::harness::TestApp;
use crate::*;

const WRITERS: usize = 24;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_writes_all_succeed_exactly_once() -> Result<(), Box<dyn core::error::Error>> {
    concurrent_writes(TestApp::new().await?).await
}

/// The service runs against files, so exercise that mode too.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_writes_all_succeed_exactly_once_on_disk()
-> Result<(), Box<dyn core::error::Error>> {
    let dir = tempfile::tempdir()?;
    concurrent_writes(TestApp::with_data_dir_and_mode(dir, DatabaseMode::File).await?).await
}

async fn concurrent_writes(app: TestApp) -> Result<(), Box<dyn core::error::Error>> {
    let app = app
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();
    let agent = AgentName::new("thinker.process");

    client
        .continuity()
        .emerge(
            &EmergeAgent::builder_v1()
                .name("thinker")
                .persona("process")
                .build()
                .into(),
        )
        .await?;

    let writes = (0..WRITERS).map(|n| {
        let client = app.client();
        let agent = agent.clone();
        tokio::spawn(async move {
            client
                .cognition()
                .add(
                    &AddCognition::builder_v1()
                        .agent(agent)
                        .texture("observation")
                        .content(format!("concurrent thought {n}"))
                        .build()
                        .into(),
                )
                .await
                .map(|_| ())
                .map_err(|error| format!("write {n}: {error}"))
        })
    });

    let mut failures = Vec::new();
    for write in futures::future::join_all(writes).await {
        if let Err(failure) = write? {
            failures.push(failure);
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {WRITERS} concurrent writes failed: {failures:#?}",
        failures.len()
    );

    match client
        .cognition()
        .list(&ListCognitions::builder_v1().agent(agent).build().into())
        .await?
    {
        CognitionResponse::Cognitions(CognitionsResponse::V1(listed)) => {
            assert_eq!(listed.total, WRITERS, "each write lands exactly once");
        }
        other => panic!("expected Cognitions, got {other:?}"),
    }

    Ok(())
}
