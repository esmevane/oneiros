use crate::*;

use super::*;

#[test]
fn wake_hints_include_guidebook() {
    let hints = WakeHints::builder()
        .agent(AgentName::new("governor.process"))
        .build()
        .hints();
    assert_eq!(hints.len(), 3);
    assert!(hints[0].action.contains("guidebook"));
    assert_eq!(hints[0].level, HintLevel::FollowUp);
}

#[test]
fn wake_hints_promote_under_pressure() {
    let hints = WakeHints::builder()
        .agent(AgentName::new("governor.process"))
        .pressures(vec![PressureSummary {
            urge: UrgeName::new("introspect"),
            percent: 80,
        }])
        .build()
        .hints();
    let suggest_count = hints
        .iter()
        .filter(|h| h.level == HintLevel::Suggest)
        .count();
    assert_eq!(suggest_count, 0, "all Suggest should be promoted");
}

#[test]
fn wake_hints_no_promotion_below_threshold() {
    let hints = WakeHints::builder()
        .agent(AgentName::new("governor.process"))
        .pressures(vec![PressureSummary {
            urge: UrgeName::new("introspect"),
            percent: 30,
        }])
        .build()
        .hints();
    let suggest_count = hints
        .iter()
        .filter(|h| h.level == HintLevel::Suggest)
        .count();
    assert!(suggest_count > 0, "low pressure should leave Suggest alone");
}

#[test]
fn reflect_hints_suggest_consolidation() {
    let hints = ReflectHints::builder()
        .agent(AgentName::new("governor.process"))
        .build()
        .hints();
    assert_eq!(hints.len(), 3);
    assert!(hints[0].action.contains("memory add"));
}

#[test]
fn cognition_added_hints_include_ref_token() {
    let ref_token = RefToken::new(Ref::cognition(CognitionId::new()));
    let hints = CognitionAddedHints::builder()
        .agent(AgentName::new("governor.process"))
        .ref_token(ref_token.clone())
        .build()
        .hints();
    assert_eq!(hints.len(), 3);
    assert!(hints[2].action.contains(&ref_token.to_string()));
}

#[test]
fn mutation_hints_include_ref_token() {
    let ref_token = RefToken::new(Ref::memory(MemoryId::new()));
    let hints = MutationHints::builder()
        .ref_token(ref_token.clone())
        .build()
        .hints();
    assert_eq!(hints.len(), 2);
    assert!(hints[0].action.contains(&ref_token.to_string()));
}

#[test]
fn listing_hints_grow_when_has_more() {
    let agent = AgentName::new("governor.process");
    let without = ListingHints::builder().agent(agent.clone()).build().hints();
    let with = ListingHints::builder()
        .agent(agent)
        .has_more(true)
        .build()
        .hints();
    assert_eq!(without.len(), 1);
    assert_eq!(with.len(), 2);
}

#[test]
fn hint_set_delegates_to_inner() {
    let set = HintSet::reflect(
        ReflectHints::builder()
            .agent(AgentName::new("test"))
            .build(),
    );
    let hints = set.hints();
    assert_eq!(hints.len(), 3);
}

#[test]
fn hint_template_renders_empty_as_empty() {
    let template = HintTemplate { hints: &[] };
    let rendered = template.to_string();
    assert!(rendered.trim().is_empty());
}

#[test]
fn hint_template_renders_markdown() {
    let hints = vec![
        Hint::follow_up(
            "introspect governor.process",
            "Cognitive pressure is building",
        ),
        Hint::suggest("reflect governor.process", "Pause on something significant"),
    ];
    let template = HintTemplate { hints: &hints };
    let rendered = template.to_string();
    assert!(rendered.contains("## Hints"));
    assert!(rendered.contains("**follow-up** `introspect governor.process`"));
    assert!(rendered.contains("**suggest** `reflect governor.process`"));
}
