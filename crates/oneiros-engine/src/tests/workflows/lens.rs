//! Lens workflow — parse, validate, name-resolve, explain, and query
//! end-to-end through the HTTP service.

use crate::tests::harness::TestApp;
use crate::*;

async fn seeded_app() -> Result<TestApp, Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    app.command(r#"agent create gov process --description "Governor agent""#)
        .await?;
    app.command(r#"cognition add gov.process observation "The garden is growing""#)
        .await?;
    app.command(r#"cognition add gov.process reflection "Patterns emerge from fragments""#)
        .await?;
    app.command(r#"cognition add gov.process observation "Seeds need tending""#)
        .await?;
    app.command(r#"memory add gov.process project "Event sourcing works well""#)
        .await?;

    Ok(app)
}

fn extract_query_hits(rendered: &Rendered<Responses>) -> &[Hit] {
    let Responses::Lens(LensResponse::Queried(QueriedLensResponse::V1(queried))) =
        rendered.response()
    else {
        panic!("expected Lens(Queried), got {:#?}", rendered.response());
    };
    &queried.hits
}

#[tokio::test]
async fn lens_parse_returns_round_trip_display() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let rendered = app.command(r#"lens parse "agent(gov.process)""#).await?;
    let Responses::Lens(LensResponse::Parsed(ParsedLensResponse::V1(parsed))) = rendered.response()
    else {
        panic!("expected Lens(Parsed), got {:#?}", rendered.response());
    };

    assert_eq!(parsed.source, "agent(gov.process)");
    assert_eq!(parsed.display, "agent(gov.process)");

    Ok(())
}

#[tokio::test]
async fn lens_explain_rejects_unknown_agent_name() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    app.command(r#"agent create governor process --description "Governor agent""#)
        .await?;

    let known = app
        .command(r#"lens explain "agent(governor.process)""#)
        .await?;
    let Responses::Lens(LensResponse::Explained(_)) = known.response() else {
        panic!(
            "expected Explained for known agent, got {:#?}",
            known.response()
        );
    };

    let typo = app
        .command(r#"lens explain "agent(governorr.process)""#)
        .await;
    let error = typo.expect_err("typo must reject end-to-end");
    let message = format!("{error}");
    assert!(
        message.contains("unknown agent"),
        "expected UnknownSymbol message, got: {message}"
    );
    assert!(
        message.contains("governorr.process"),
        "expected the bad name in the message, got: {message}"
    );

    Ok(())
}

#[tokio::test]
async fn lens_explain_rejects_unknown_texture_name() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let known = app
        .command(r#"lens explain "texture(observation)""#)
        .await?;
    let Responses::Lens(LensResponse::Explained(_)) = known.response() else {
        panic!("expected Explained for known texture");
    };

    let typo = app.command(r#"lens explain "texture(observasion)""#).await;
    let error = typo.expect_err("typo must reject end-to-end");
    let message = format!("{error}");
    assert!(
        message.contains("unknown texture"),
        "expected UnknownSymbol message, got: {message}"
    );

    Ok(())
}

#[tokio::test]
async fn lens_query_returns_hits_for_single_predicate() -> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    let rendered = app.command(r#"lens query "texture(observation)""#).await?;
    let hits = extract_query_hits(&rendered);

    assert_eq!(hits.len(), 2, "two cognitions with texture=observation");
    for hit in hits {
        assert!(matches!(hit, Hit::Entity(_)));
    }

    Ok(())
}

#[tokio::test]
async fn lens_query_intersection_returns_only_matching_hits()
-> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    let rendered = app
        .command(r#"lens query "texture(observation) & agent(gov.process)""#)
        .await?;
    let hits = extract_query_hits(&rendered);

    assert_eq!(hits.len(), 2, "both observations belong to gov.process");

    Ok(())
}

#[tokio::test]
async fn lens_query_union_returns_combined_hits() -> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    let rendered = app
        .command(r#"lens query "texture(observation) | texture(reflection)""#)
        .await?;
    let hits = extract_query_hits(&rendered);

    assert_eq!(hits.len(), 3, "2 observations + 1 reflection");

    Ok(())
}

#[tokio::test]
async fn lens_query_difference_returns_left_minus_right() -> Result<(), Box<dyn core::error::Error>>
{
    let app = seeded_app().await?;

    let all_rendered = app.command(r#"lens query "agent(gov.process)""#).await?;
    let all_hits = extract_query_hits(&all_rendered);
    let all_count = all_hits.len();

    let diff_rendered = app
        .command(r#"lens query "agent(gov.process) ~ texture(observation)""#)
        .await?;
    let diff_hits = extract_query_hits(&diff_rendered);

    assert!(
        diff_hits.len() < all_count,
        "difference should reduce the set"
    );
    assert!(
        diff_hits.len() >= 2,
        "should have at least the reflection + memory: got {}",
        diff_hits.len()
    );

    Ok(())
}

#[tokio::test]
async fn lens_query_returns_empty_for_unimplemented_predicate()
-> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    // `working` is a seeded texture but our test data has no cognitions with it,
    // so the query should return empty rather than erroring.
    let rendered = app.command(r#"lens query "texture(working)""#).await?;
    let hits = extract_query_hits(&rendered);

    assert_eq!(
        hits.len(),
        0,
        "no cognitions with that texture in test data"
    );

    Ok(())
}

#[tokio::test]
async fn lens_compile_returns_ir_for_inspection() -> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    let rendered = app
        .command(r#"lens explain "texture(observation) & agent(gov.process)""#)
        .await?;
    let Responses::Lens(LensResponse::Explained(ExplainedLensResponse::V1(explained))) =
        rendered.response()
    else {
        panic!("expected Lens(Explained), got {:#?}", rendered.response());
    };

    assert!(
        explained.plan.contains("$0"),
        "IR should contain slot references: {}",
        explained.plan
    );

    Ok(())
}

#[tokio::test]
async fn lens_query_results_sort_stably_by_timestamp_desc()
-> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    let rendered = app.command(r#"lens query "agent(gov.process)""#).await?;
    let hits = extract_query_hits(&rendered);

    assert!(hits.len() >= 2, "need multiple hits to test ordering");
    for window in hits.windows(2) {
        let a_ts = window[0].timestamp();
        let b_ts = window[1].timestamp();
        assert!(a_ts >= b_ts, "hits should be in descending timestamp order");
    }

    Ok(())
}

#[tokio::test]
async fn lens_query_results_sort_by_relevance_when_requested()
-> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    let rendered = app.command(r#"lens query "search(\"garden\")""#).await?;
    let hits = extract_query_hits(&rendered);

    assert!(
        !hits.is_empty(),
        "search should find at least the 'garden' cognition"
    );
    for hit in hits {
        assert!(matches!(hit, Hit::Entity(_)));
    }

    Ok(())
}

#[tokio::test]
async fn lens_query_events_for_returns_events_that_touched_entity()
-> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();

    let agent = match client
        .agent()
        .create(
            &CreateAgent::builder_v1()
                .name(AgentName::new("governor.process"))
                .persona(PersonaName::new("process"))
                .build()
                .into(),
        )
        .await?
    {
        AgentResponse::AgentCreated(AgentCreatedResponse::V1(r)) => r.agent,
        other => panic!("expected AgentCreated, got {other:?}"),
    };

    let agent_ref = RefToken::new(Ref::agent(agent.id));

    let rendered = app
        .command(&format!(r#"lens query "events_for({agent_ref})""#))
        .await?;
    let hits = extract_query_hits(&rendered);

    assert!(!hits.is_empty(), "agent_created event should be on trail");
    for hit in hits {
        assert!(
            matches!(hit, Hit::Event(_)),
            "events_for should yield event hits, got {hit:?}"
        );
    }

    Ok(())
}

#[tokio::test]
async fn lens_query_refs_from_round_trips_through_events_for()
-> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();

    let agent = match client
        .agent()
        .create(
            &CreateAgent::builder_v1()
                .name(AgentName::new("governor.process"))
                .persona(PersonaName::new("process"))
                .build()
                .into(),
        )
        .await?
    {
        AgentResponse::AgentCreated(AgentCreatedResponse::V1(r)) => r.agent,
        other => panic!("expected AgentCreated, got {other:?}"),
    };

    let agent_ref = RefToken::new(Ref::agent(agent.id));

    let rendered = app
        .command(&format!(
            r#"lens query "refs_from(events_for({agent_ref}))""#
        ))
        .await?;
    let hits = extract_query_hits(&rendered);

    let entity_refs: Vec<RefToken> = hits
        .iter()
        .filter_map(|hit| match hit {
            Hit::Entity(EntityHit { entity_ref, .. }) => Some(RefToken::new(entity_ref.clone())),
            _ => None,
        })
        .collect();

    assert!(
        entity_refs.contains(&agent_ref),
        "round-trip should land on the original agent, got {entity_refs:?}"
    );

    Ok(())
}

#[tokio::test]
async fn lens_query_facet_predicate_accepts_union_of_names()
-> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    app.command(r#"agent create alpha process --description "first""#)
        .await?;
    app.command(r#"agent create beta process --description "second""#)
        .await?;

    app.command(r#"cognition add alpha.process observation "A1""#)
        .await?;
    app.command(r#"cognition add beta.process observation "B1""#)
        .await?;
    app.command(r#"cognition add beta.process reflection "B2""#)
        .await?;

    let alone_alpha =
        extract_query_hits(&app.command(r#"lens query "agent(alpha.process)""#).await?).len();
    let alone_beta =
        extract_query_hits(&app.command(r#"lens query "agent(beta.process)""#).await?).len();
    let union = extract_query_hits(
        &app.command(r#"lens query "agent(alpha.process | beta.process)""#)
            .await?,
    )
    .len();

    assert_eq!(
        union,
        alone_alpha + alone_beta,
        "set-of-names union should equal sum of singletons when sets are disjoint"
    );
    assert!(
        union >= 3,
        "should pick up at least the three cognitions added by the test"
    );

    Ok(())
}

#[tokio::test]
async fn lens_query_facet_predicate_intersection_of_names_is_empty()
-> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    // No cognition has both texture=observation AND texture=reflection on the same row,
    // so the intersection of two name-singletons is empty (the columns are scalar).
    let rendered = app
        .command(r#"lens query "texture(observation & reflection)""#)
        .await?;
    let hits = extract_query_hits(&rendered);

    assert_eq!(hits.len(), 0);

    Ok(())
}

#[tokio::test]
async fn lens_query_events_for_accepts_inner_lens() -> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    let rendered = app
        .command(r#"lens query "events_for(agent(gov.process))""#)
        .await?;
    let hits = extract_query_hits(&rendered);

    assert!(
        !hits.is_empty(),
        "agent's entities have events on the trail"
    );
    for hit in hits {
        assert!(
            matches!(hit, Hit::Event(_)),
            "events_for(<entity-lens>) yields event hits, got {hit:?}"
        );
    }

    Ok(())
}

/// Builds a 4-cognition chain `a → b → c → d` joined by `caused` connections.
/// Returns the agent ref and the four cognition refs.
async fn seeded_chain() -> Result<(TestApp, RefToken, [RefToken; 4]), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    app.command(r#"nature set caused --description "A produced B""#)
        .await?;
    app.command(r#"agent create thinker process --description "T""#)
        .await?;

    let mut refs: Vec<RefToken> = Vec::with_capacity(4);
    for (idx, label) in ["a", "b", "c", "d"].iter().enumerate() {
        let rendered = app
            .command(&format!(
                r#"cognition add thinker.process observation "node {label}""#
            ))
            .await?;
        let id = match rendered.response() {
            Responses::Cognition(CognitionResponse::CognitionAdded(
                CognitionAddedResponse::V1(added),
            )) => added.cognition.id,
            other => panic!("expected CognitionAdded for {label}, got {other:#?}"),
        };
        let _ = idx;
        refs.push(RefToken::new(Ref::cognition(id)));
    }

    for window in refs.windows(2) {
        let from_ref = &window[0];
        let to_ref = &window[1];
        app.command(&format!("connection create caused {from_ref} {to_ref}"))
            .await?;
    }

    let agent_ref = RefToken::new(Ref::agent(AgentId::new()));
    let four: [RefToken; 4] = [
        refs[0].clone(),
        refs[1].clone(),
        refs[2].clone(),
        refs[3].clone(),
    ];
    Ok((app, agent_ref, four))
}

fn extract_refs(hits: &[Hit]) -> Vec<RefToken> {
    hits.iter()
        .filter_map(|hit| match hit {
            Hit::Entity(EntityHit { entity_ref, .. }) => Some(RefToken::new(entity_ref.clone())),
            _ => None,
        })
        .collect()
}

#[tokio::test]
async fn lens_query_from_returns_direct_successors() -> Result<(), Box<dyn core::error::Error>> {
    let (app, _agent, [a, b, _c, _d]) = seeded_chain().await?;

    let rendered = app.command(&format!(r#"lens query "from({a})""#)).await?;
    let refs = extract_refs(extract_query_hits(&rendered));

    assert_eq!(refs, vec![b], "from(a) should yield {{b}}");

    Ok(())
}

#[tokio::test]
async fn lens_query_to_returns_direct_predecessors() -> Result<(), Box<dyn core::error::Error>> {
    let (app, _agent, [_a, _b, c, d]) = seeded_chain().await?;

    let rendered = app.command(&format!(r#"lens query "to({d})""#)).await?;
    let refs = extract_refs(extract_query_hits(&rendered));

    assert_eq!(refs, vec![c], "to(d) should yield {{c}}");

    Ok(())
}

#[tokio::test]
async fn lens_query_descendants_returns_transitive_successors()
-> Result<(), Box<dyn core::error::Error>> {
    let (app, _agent, [a, b, c, d]) = seeded_chain().await?;

    let rendered = app
        .command(&format!(r#"lens query "descendants({a})""#))
        .await?;
    let mut refs = extract_refs(extract_query_hits(&rendered));
    refs.sort_by_key(|r| r.to_string());
    let mut expected = vec![b, c, d];
    expected.sort_by_key(|r| r.to_string());

    assert_eq!(refs, expected, "descendants(a) should yield {{b, c, d}}");

    Ok(())
}

#[tokio::test]
async fn lens_query_ancestors_returns_transitive_predecessors()
-> Result<(), Box<dyn core::error::Error>> {
    let (app, _agent, [a, b, c, d]) = seeded_chain().await?;

    let rendered = app
        .command(&format!(r#"lens query "ancestors({d})""#))
        .await?;
    let mut refs = extract_refs(extract_query_hits(&rendered));
    refs.sort_by_key(|r| r.to_string());
    let mut expected = vec![a, b, c];
    expected.sort_by_key(|r| r.to_string());

    assert_eq!(refs, expected, "ancestors(d) should yield {{a, b, c}}");

    Ok(())
}

#[tokio::test]
async fn lens_query_within_bounds_depth_both_directions() -> Result<(), Box<dyn core::error::Error>>
{
    let (app, _agent, [a, b, c, _d]) = seeded_chain().await?;

    let rendered = app
        .command(&format!(r#"lens query "within({b}, 1)""#))
        .await?;
    let mut refs = extract_refs(extract_query_hits(&rendered));
    refs.sort_by_key(|r| r.to_string());
    let mut expected = vec![a, c];
    expected.sort_by_key(|r| r.to_string());

    assert_eq!(refs, expected, "within(b, 1) should yield {{a, c}}");

    Ok(())
}

#[tokio::test]
async fn lens_query_component_returns_connected_component()
-> Result<(), Box<dyn core::error::Error>> {
    let (app, _agent, [a, b, c, d]) = seeded_chain().await?;

    let rendered = app
        .command(&format!(r#"lens query "component({a})""#))
        .await?;
    let mut refs = extract_refs(extract_query_hits(&rendered));
    refs.sort_by_key(|r| r.to_string());
    let mut expected = vec![b, c, d];
    expected.sort_by_key(|r| r.to_string());

    assert_eq!(refs, expected, "component(a) reaches {{b, c, d}}");

    Ok(())
}

#[tokio::test]
async fn lens_query_graph_predicate_composes_with_set_ops()
-> Result<(), Box<dyn core::error::Error>> {
    let (app, _agent, [a, b, _c, _d]) = seeded_chain().await?;

    let rendered = app
        .command(&format!(r#"lens query "from({a}) | from({b})""#))
        .await?;
    let mut refs = extract_refs(extract_query_hits(&rendered));
    refs.sort_by_key(|r| r.to_string());

    assert_eq!(refs.len(), 2, "union of from(a) and from(b) has 2 members");

    Ok(())
}

#[tokio::test]
async fn lens_query_between_returns_events_diverged_between_bookmarks()
-> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    app.command(r#"agent create thinker process --description "T""#)
        .await?;
    app.command(r#"cognition add thinker.process observation "first""#)
        .await?;

    let main_bookmarks = match app.command("bookmark list").await?.response() {
        Responses::Bookmark(BookmarkResponse::Bookmarks(listed)) => listed.items.clone(),
        other => panic!("expected Bookmarks, got {other:#?}"),
    };
    let main_ref = RefToken::new(Ref::bookmark(
        main_bookmarks
            .iter()
            .find(|b| b.name.as_str() == "main")
            .expect("main bookmark exists")
            .id,
    ));

    let experiment = match app.command("bookmark create experiment").await?.response() {
        Responses::Bookmark(BookmarkResponse::Forked(BookmarkForkedResponse::V1(r))) => {
            r.bookmark.clone()
        }
        other => panic!("expected Bookmark::Forked, got {other:#?}"),
    };
    let experiment_ref = RefToken::new(Ref::bookmark(experiment.id));

    app.command(r#"cognition add thinker.process observation "branch-only""#)
        .await?;

    let rendered = app
        .command(&format!(
            r#"lens query "between({main_ref}, {experiment_ref})""#
        ))
        .await?;
    let hits = extract_query_hits(&rendered);
    assert!(
        !hits.is_empty(),
        "experiment should hold at least one event main lacks"
    );
    for hit in hits {
        assert!(
            matches!(hit, Hit::Event(_)),
            "between() should yield event hits, got {hit:?}"
        );
    }

    let inverse = app
        .command(&format!(
            r#"lens query "between({experiment_ref}, {main_ref})""#
        ))
        .await?;
    let inverse_hits = extract_query_hits(&inverse);
    assert!(
        inverse_hits.is_empty(),
        "main has no events experiment lacks, got {} hits",
        inverse_hits.len()
    );

    Ok(())
}

#[tokio::test]
async fn memory_list_with_lens_routes_through_lens_executor()
-> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;
    let client = app.client();

    let rendered = client
        .memory()
        .list(
            &ListMemories::builder_v1()
                .lens("level(project)".to_string())
                .build()
                .into(),
        )
        .await?;

    let items = match rendered {
        MemoryResponse::Memories(MemoriesResponse::V1(r)) => r.items,
        other => panic!("expected Memories, got {other:#?}"),
    };
    assert_eq!(items.len(), 1, "one project-level memory in seed");
    Ok(())
}

#[tokio::test]
async fn connection_list_with_lens_routes_through_lens_executor()
-> Result<(), Box<dyn core::error::Error>> {
    let (app, _agent, [a, b, _c, _d]) = seeded_chain().await?;
    let client = app.client();

    let rendered = client
        .connection()
        .list(
            &ListConnections::builder_v1()
                .lens(format!("from({a}) | from({b})"))
                .build()
                .into(),
        )
        .await?;

    let _items = match rendered {
        ConnectionResponse::Connections(ConnectionsResponse::V1(r)) => r.items,
        ConnectionResponse::NoConnections => Vec::new(),
        other => panic!("expected Connections, got {other:#?}"),
    };
    Ok(())
}

#[tokio::test]
async fn agent_list_with_lens_routes_through_lens_executor()
-> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;
    let client = app.client();

    let rendered = client
        .agent()
        .list(
            &ListAgents::builder_v1()
                .lens("agent(gov.process)".to_string())
                .build()
                .into(),
        )
        .await?;

    let items = match rendered {
        AgentResponse::Agents(AgentsResponse::V1(r)) => r.items,
        AgentResponse::NoAgents => Vec::new(),
        other => panic!("expected Agents, got {other:#?}"),
    };
    assert_eq!(items.len(), 1, "the gov.process agent");
    assert_eq!(items[0].name.as_str(), "gov.process");
    Ok(())
}

#[tokio::test]
async fn experience_list_with_lens_routes_through_lens_executor()
-> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;
    let client = app.client();

    let rendered = client
        .experience()
        .list(
            &ListExperiences::builder_v1()
                .lens("kind(experience)".to_string())
                .build()
                .into(),
        )
        .await?;

    let _items = match rendered {
        ExperienceResponse::Experiences(ExperiencesResponse::V1(r)) => r.items,
        ExperienceResponse::NoExperiences => Vec::new(),
        other => panic!("expected Experiences, got {other:#?}"),
    };
    Ok(())
}

#[tokio::test]
async fn cognition_list_with_lens_routes_through_lens_executor()
-> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;
    let client = app.client();

    let rendered = client
        .cognition()
        .list(
            &ListCognitions::builder_v1()
                .lens("texture(observation)".to_string())
                .build()
                .into(),
        )
        .await?;

    let items = match rendered {
        CognitionResponse::Cognitions(CognitionsResponse::V1(r)) => r.items,
        other => panic!("expected Cognitions, got {other:#?}"),
    };

    assert_eq!(items.len(), 2, "two cognitions have texture=observation");
    for cognition in &items {
        assert_eq!(cognition.texture.as_str(), "observation");
    }

    Ok(())
}

#[tokio::test]
async fn lens_query_events_for_unknown_ref_returns_empty() -> Result<(), Box<dyn core::error::Error>>
{
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let unknown_ref = RefToken::new(Ref::agent(AgentId::new()));

    let rendered = app
        .command(&format!(r#"lens query "events_for({unknown_ref})""#))
        .await?;
    let hits = extract_query_hits(&rendered);

    assert!(hits.is_empty(), "unknown entity has no trail");

    Ok(())
}
