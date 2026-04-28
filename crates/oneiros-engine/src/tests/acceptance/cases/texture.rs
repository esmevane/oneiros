use super::*;

pub(crate) async fn set_creates<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness
        .exec_json("texture set observation --description 'Noticing patterns' --prompt 'Use when you see something interesting.'")
        .await?;

    assert!(
        matches!(response, Responses::Texture(TextureResponse::TextureSet(_))),
        "expected TextureSet, got {response:#?}"
    );

    let show_response = harness.exec_json("texture show observation").await?;

    match show_response {
        Responses::Texture(TextureResponse::TextureDetails(TextureDetailsResponse::V1(
            details,
        ))) => {
            assert_eq!(details.texture.name.as_str(), "observation");
            assert_eq!(details.texture.description.as_str(), "Noticing patterns");
        }
        other => panic!("expected TextureDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn set_updates<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("texture set draft --description 'Original' --prompt 'Original.'")
        .await?;

    harness
        .exec_json("texture set draft --description 'Updated' --prompt 'Updated.'")
        .await?;

    let show_response = harness.exec_json("texture show draft").await?;

    match show_response {
        Responses::Texture(TextureResponse::TextureDetails(TextureDetailsResponse::V1(
            details,
        ))) => {
            assert_eq!(details.texture.description.as_str(), "Updated");
        }
        other => panic!("expected TextureDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness.exec_json("texture list").await?;

    assert!(
        matches!(response, Responses::Texture(TextureResponse::NoTextures)),
        "expected NoTextures, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("texture set first --description 'First' --prompt 'First.'")
        .await?;

    harness
        .exec_json("texture set second --description 'Second' --prompt 'Second.'")
        .await?;

    let response = harness.exec_json("texture list").await?;

    match response {
        Responses::Texture(TextureResponse::Textures(TexturesResponse::V1(list))) => {
            assert_eq!(list.items.len(), 2);
        }
        other => panic!("expected Textures, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn remove<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("texture set temporary --description 'Will be removed' --prompt 'Temporary.'")
        .await?;

    let remove_response = harness.exec_json("texture remove temporary").await?;

    assert!(
        matches!(
            remove_response,
            Responses::Texture(TextureResponse::TextureRemoved(_))
        ),
        "expected TextureRemoved, got {remove_response:?}"
    );

    let list_response = harness.exec_json("texture list").await?;

    assert!(
        matches!(
            list_response,
            Responses::Texture(TextureResponse::NoTextures)
        ),
        "expected NoTextures after removal, got {list_response:?}"
    );

    Ok(())
}

pub(crate) async fn set_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let prompt = harness
        .exec_prompt(
            "texture set test-texture --description 'A test texture' --prompt 'Test prompt.'",
        )
        .await?;

    assert!(!prompt.is_empty(), "texture set prompt should not be empty");

    Ok(())
}

pub(crate) async fn show_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json(
            "texture set test-texture --description 'A test texture' --prompt 'Test prompt.'",
        )
        .await?;

    let prompt = harness.exec_prompt("texture show test-texture").await?;

    assert!(
        !prompt.is_empty(),
        "texture show prompt should not be empty"
    );
    assert!(
        prompt.contains("test-texture"),
        "texture show prompt should contain the entry name"
    );

    Ok(())
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json(
            "texture set test-texture --description 'A test texture' --prompt 'Test prompt.'",
        )
        .await?;

    let prompt = harness.exec_prompt("texture list").await?;

    assert!(
        !prompt.is_empty(),
        "texture list prompt should not be empty when entries exist"
    );

    Ok(())
}

pub(crate) async fn remove_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("texture set temporary --description 'Will be removed' --prompt 'Temporary.'")
        .await?;

    let prompt = harness.exec_prompt("texture remove temporary").await?;

    assert!(
        !prompt.is_empty(),
        "texture remove prompt should not be empty"
    );

    Ok(())
}
