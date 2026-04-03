use oneiros_engine::*;
use oneiros_usage::*;

pub(crate) async fn list_after_system_init<B: Backend>() -> TestResult {
    let harness = Harness::<B>::setup_system().await?;

    let response = harness.exec_json("actor list").await?;

    match response {
        Responses::Actor(ActorResponse::Listed(actors)) => {
            assert_eq!(
                actors.len(),
                1,
                "system init should create exactly one actor"
            );
            assert_eq!(actors.items[0].data.name.as_str(), "test");
        }
        other => panic!("expected Actor(Listed), got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::setup_system().await?;

    let prompt = harness.exec_prompt("actor list").await?;

    assert!(!prompt.is_empty(), "actor list prompt should not be empty");
    assert!(
        prompt.contains("1 found"),
        "actor list prompt should describe the actor count, got: {prompt}"
    );

    Ok(())
}
