use oneiros_model::*;
use std::collections::BTreeMap;

pub(crate) fn cognition_gauge(
    agent: &AgentName,
    cognitions: &[Identity<CognitionId, Cognition>],
) -> String {
    let breakdown = group_by(cognitions, |c| c.texture.to_string());
    format_gauge(agent, "cognitions", &breakdown)
}

pub(crate) fn memory_gauge(agent: &AgentName, memories: &[Identity<MemoryId, Memory>]) -> String {
    let breakdown = group_by(memories, |m| m.level.to_string());
    format_gauge(agent, "memories", &breakdown)
}

pub(crate) fn experience_gauge(
    agent: &AgentName,
    experiences: &[Identity<ExperienceId, Experience>],
) -> String {
    let breakdown = group_by(experiences, |e| e.sensation.to_string());
    format_gauge(agent, "experiences", &breakdown)
}

pub(crate) fn full_status(
    agent: &AgentName,
    cognitions: &[Identity<CognitionId, Cognition>],
    memories: &[Identity<MemoryId, Memory>],
    experiences: &[Identity<ExperienceId, Experience>],
) -> String {
    let cog_breakdown = group_by(cognitions, |c| c.texture.to_string());
    let mem_breakdown = group_by(memories, |m| m.level.to_string());
    let exp_breakdown = group_by(experiences, |e| e.sensation.to_string());

    let mut lines = vec![format!("Brain: {agent}")];

    lines.push(String::new());
    lines.push(format!("Cognitions: {} total", cognitions.len()));
    if !cog_breakdown.is_empty() {
        lines.push(format!("  {}", format_breakdown(&cog_breakdown)));
    }

    lines.push(String::new());
    lines.push(format!("Memories: {} total", memories.len()));
    if !mem_breakdown.is_empty() {
        lines.push(format!("  {}", format_breakdown(&mem_breakdown)));
    }

    lines.push(String::new());
    lines.push(format!("Experiences: {} total", experiences.len()));
    if !exp_breakdown.is_empty() {
        lines.push(format!("  {}", format_breakdown(&exp_breakdown)));
    }

    lines.join("\n")
}

fn group_by<T>(items: &[T], key: impl Fn(&T) -> String) -> Vec<(String, usize)> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for item in items {
        *counts.entry(key(item)).or_default() += 1;
    }
    let mut pairs: Vec<_> = counts.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    pairs
}

fn format_breakdown(breakdown: &[(String, usize)]) -> String {
    let parts: Vec<String> = breakdown
        .iter()
        .take(5)
        .map(|(k, v)| format!("{v} {k}"))
        .collect();
    let suffix = if breakdown.len() > 5 { ", ..." } else { "" };
    format!("{}{suffix}", parts.join(", "))
}

fn format_gauge(agent: &AgentName, entity_name: &str, breakdown: &[(String, usize)]) -> String {
    let total: usize = breakdown.iter().map(|(_, n)| n).sum();
    format!(
        "[{agent} · {total} {entity_name} · {}]",
        format_breakdown(breakdown)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn make_cognition(texture: &str) -> Identity<CognitionId, Cognition> {
        Identity::new(
            CognitionId::new(),
            Cognition {
                agent_id: AgentId::new(),
                texture: TextureName::new(texture),
                content: Content::new("test"),
                created_at: Utc::now(),
            },
        )
    }

    fn make_memory(level: &str) -> Identity<MemoryId, Memory> {
        Identity::new(
            MemoryId::new(),
            Memory {
                agent_id: AgentId::new(),
                level: LevelName::new(level),
                content: Content::new("test"),
                created_at: Utc::now(),
            },
        )
    }

    fn make_experience(sensation: &str) -> Identity<ExperienceId, Experience> {
        Identity::new(
            ExperienceId::new(),
            Experience {
                agent_id: AgentId::new(),
                sensation: SensationName::new(sensation),
                description: Description::new("test"),
                refs: vec![],
                created_at: Utc::now(),
            },
        )
    }

    #[test]
    fn cognition_gauge_shows_counts_by_texture() {
        let agent = AgentName::new("governor.process");
        let cognitions = vec![
            make_cognition("working"),
            make_cognition("working"),
            make_cognition("observation"),
        ];
        let gauge = cognition_gauge(&agent, &cognitions);
        assert_eq!(
            gauge,
            "[governor.process · 3 cognitions · 2 working, 1 observation]"
        );
    }

    #[test]
    fn memory_gauge_shows_counts_by_level() {
        let agent = AgentName::new("governor.process");
        let memories = vec![
            make_memory("session"),
            make_memory("project"),
            make_memory("session"),
        ];
        let gauge = memory_gauge(&agent, &memories);
        assert_eq!(
            gauge,
            "[governor.process · 3 memories · 2 session, 1 project]"
        );
    }

    #[test]
    fn experience_gauge_shows_counts_by_sensation() {
        let agent = AgentName::new("governor.process");
        let experiences = vec![
            make_experience("echoes"),
            make_experience("continues"),
            make_experience("echoes"),
        ];
        let gauge = experience_gauge(&agent, &experiences);
        assert_eq!(
            gauge,
            "[governor.process · 3 experiences · 2 echoes, 1 continues]"
        );
    }

    #[test]
    fn gauge_truncates_at_five_categories() {
        let agent = AgentName::new("test");
        let cognitions = vec![
            make_cognition("a"),
            make_cognition("b"),
            make_cognition("c"),
            make_cognition("d"),
            make_cognition("e"),
            make_cognition("f"),
        ];
        let gauge = cognition_gauge(&agent, &cognitions);
        assert!(gauge.ends_with(", ...]"), "Expected ellipsis, got: {gauge}");
        // Should contain exactly 5 categories plus ellipsis
        assert!(gauge.contains("1 a"));
        assert!(gauge.contains("1 e"));
        assert!(!gauge.contains("1 f"));
    }

    #[test]
    fn gauge_with_empty_list() {
        let agent = AgentName::new("test");
        let gauge = cognition_gauge(&agent, &[]);
        assert_eq!(gauge, "[test · 0 cognitions · ]");
    }

    #[test]
    fn full_status_shows_dashboard() {
        let agent = AgentName::new("governor.process");
        let cognitions = vec![make_cognition("working"), make_cognition("observation")];
        let memories = vec![make_memory("session")];
        let experiences = vec![make_experience("echoes")];

        let status = full_status(&agent, &cognitions, &memories, &experiences);
        assert!(status.contains("Brain: governor.process"));
        assert!(status.contains("Cognitions: 2 total"));
        assert!(status.contains("Memories: 1 total"));
        assert!(status.contains("Experiences: 1 total"));
    }

    #[test]
    fn group_by_sorts_by_count_descending() {
        let items = vec![
            make_cognition("observation"),
            make_cognition("working"),
            make_cognition("working"),
            make_cognition("working"),
            make_cognition("observation"),
        ];
        let groups = group_by(&items, |c| c.texture.to_string());
        assert_eq!(groups[0].0, "working");
        assert_eq!(groups[0].1, 3);
        assert_eq!(groups[1].0, "observation");
        assert_eq!(groups[1].1, 2);
    }
}
