use oneiros_engine::*;
use oneiros_usage::*;

pub(crate) async fn list_after_project_init<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness.exec_json("brain list").await?;

    match response {
        Responses::Brain(BrainResponse::Listed(brains)) => {
            assert_eq!(
                brains.len(),
                1,
                "project init should create exactly one brain"
            );
            assert_eq!(brains.items[0].data.name.as_str(), "test-project");
        }
        other => panic!("expected Brain(Listed), got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn get_by_name<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness.exec_json("brain get test-project").await?;

    match response {
        Responses::Brain(BrainResponse::Found(brain)) => {
            assert_eq!(brain.data.name.as_str(), "test-project");
        }
        other => panic!("expected Brain(Found), got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let prompt = harness.exec_prompt("brain list").await?;

    assert!(!prompt.is_empty(), "brain list prompt should not be empty");
    assert!(
        prompt.contains("1 of"),
        "brain list prompt should describe the brain count, got: {prompt}"
    );

    Ok(())
}
