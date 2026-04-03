use oneiros_engine::*;
use oneiros_usage::*;

pub(crate) async fn list_after_project_init<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness.exec_json("ticket list").await?;

    match response.data {
        Responses::Ticket(TicketResponse::Listed(tickets)) => {
            assert!(
                !tickets.is_empty(),
                "project init should create at least one ticket"
            );
            assert_eq!(
                tickets.items[0].brain_name,
                "test-project".into(),
                "ticket should be for the initialized brain"
            );
        }
        other => panic!("expected Ticket(Listed), got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let prompt = harness.exec_prompt("ticket list").await?;

    assert!(!prompt.is_empty(), "ticket list prompt should not be empty");

    Ok(())
}
