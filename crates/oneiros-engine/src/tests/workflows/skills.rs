//! Skill documentation references valid CLI commands.
//!
//! Every `` `oneiros …` `` invocation that ships in skill markdown must
//! resolve to a live subcommand. When a command path moves or gets
//! removed, this test aggregates every stale reference into one report
//! so doc drift is fixed in one pass instead of N flaky CI runs.

use clap::{Parser, error::ErrorKind};

use crate::*;

#[test]
fn skill_docs_reference_valid_commands() {
    let mut stale: Vec<StaleRef> = Vec::new();

    for skill in SkillInventory::all() {
        for invocation in extract_invocations(skill.content) {
            // The captured invocation already starts with "oneiros" — clap
            // expects argv[0] to be the binary name, so we feed it as-is.
            let Some(argv) = shlex::split(invocation) else {
                continue;
            };

            match Cli::try_parse_from(&argv) {
                Err(err) if err.kind() == ErrorKind::InvalidSubcommand => {
                    stale.push(StaleRef {
                        skill: skill.name,
                        invocation: invocation.to_string(),
                    });
                }
                // Anything else means the subcommand routed; placeholder
                // args (`<agent>`, `$ARGUMENTS`) may still fail validation,
                // but the command path itself is live.
                _ => {}
            }
        }
    }

    if !stale.is_empty() {
        let report: String = stale
            .iter()
            .map(|StaleRef { skill, invocation }| format!("  [{skill}] `{invocation}`"))
            .collect::<Vec<_>>()
            .join("\n");
        panic!(
            "{} stale command reference(s) in skill docs:\n{report}",
            stale.len()
        );
    }
}

struct StaleRef {
    skill: &'static str,
    invocation: String,
}

/// Extract `oneiros …` invocations bounded by inline backticks.
fn extract_invocations(content: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let needle = "`oneiros ";
    let mut rest = content;
    while let Some(start) = rest.find(needle) {
        let after_open = &rest[start + 1..];
        let Some(end) = after_open.find('`') else {
            break;
        };
        out.push(&after_open[..end]);
        rest = &after_open[end + 1..];
    }
    out
}
