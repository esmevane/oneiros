//! Skill documentation references valid CLI commands, and every CLI
//! command has a skill reference.
//!
//! `skill_docs_reference_valid_commands` catches docs pointing at vapor
//! (the path moved or got removed). `cli_commands_have_skill_docs` walks
//! the live CLI tree and catches commands with no shipped doc reference
//! (new or moved surface area without a corresponding skill update).
//! Together they keep the docs↔code link bidirectional.

use clap::{CommandFactory, Parser, error::ErrorKind};

use crate::*;

#[test]
fn skill_docs_reference_valid_commands() {
    let mut stale: Vec<StaleRef> = Vec::new();

    for skill in SkillInventory::all() {
        for invocation in extract_invocations(skill.content.as_ref()) {
            // The captured invocation already starts with "oneiros" — clap
            // expects argv[0] to be the binary name, so we feed it as-is.
            let Some(argv) = shlex::split(invocation) else {
                continue;
            };

            match Cli::try_parse_from(&argv) {
                Err(err) if err.kind() == ErrorKind::InvalidSubcommand => {
                    stale.push(StaleRef {
                        skill: skill.name.to_string(),
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
    skill: String,
    invocation: String,
}

#[test]
fn cli_commands_have_skill_docs() {
    let skills: Vec<Skill> = SkillInventory::all();
    let mut leaves: Vec<Vec<String>> = Vec::new();
    collect_leaves(&Cli::command(), Vec::new(), &mut leaves);

    let mut undocumented: Vec<String> = Vec::new();
    for path in &leaves {
        let invocation = format!("oneiros {}", path.join(" "));
        // A doc reference is either `` `oneiros foo bar ` `` (args follow)
        // or `` `oneiros foo bar` `` (no args). Either form counts.
        let with_args = format!("`{invocation} ");
        let bare = format!("`{invocation}`");
        let documented = skills
            .iter()
            .any(|skill| skill.content.contains(&with_args) || skill.content.contains(&bare));
        if !documented {
            undocumented.push(invocation);
        }
    }

    if !undocumented.is_empty() {
        let report: String = undocumented
            .iter()
            .map(|invocation| format!("  `{invocation}`"))
            .collect::<Vec<_>>()
            .join("\n");
        panic!(
            "{} CLI command(s) without skill documentation:\n{report}",
            undocumented.len()
        );
    }
}

/// Recurse the clap command tree, emitting one path per leaf subcommand.
/// The auto-generated `help` subcommand is skipped — it isn't part of the
/// authored surface.
fn collect_leaves(command: &clap::Command, prefix: Vec<String>, out: &mut Vec<Vec<String>>) {
    let subcommands: Vec<_> = command
        .get_subcommands()
        .filter(|sub| sub.get_name() != "help")
        .collect();

    if subcommands.is_empty() {
        if !prefix.is_empty() {
            out.push(prefix);
        }
        return;
    }

    for subcommand in subcommands {
        let mut next = prefix.clone();
        next.push(subcommand.get_name().to_string());
        collect_leaves(subcommand, next, out);
    }
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
