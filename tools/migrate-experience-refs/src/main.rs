use oneiros_db::Database;
use oneiros_model::*;
use oneiros_service::projections;

fn map_role_to_nature(role: Option<&str>) -> &'static str {
    match role {
        // Origin family: generative relationships
        Some("origin") | Some("caused") => "origin",
        Some("raw") | Some("raw-material") | Some("raw-observation") => "origin",
        Some("result") | Some("recognition") => "origin",
        Some("mechanism") | Some("produced") => "origin",
        Some("prediction") | Some("realization") => "origin",
        Some("predecessor") | Some("applied-instance") => "origin",

        // Revision family: updates and supersessions
        Some("crystallized") | Some("crystallized-form") | Some("crystallized-ordering") => {
            "revision"
        }
        Some("resolution") | Some("resolution-evidence") => "revision",
        Some("distills") | Some("simplification") => "revision",
        Some("three-layer-completion") | Some("vocabulary-completion") => "revision",

        // Context family: background and framing
        Some("foundation") | Some("grounds") => "context",
        Some("grounded") | Some("grounding-bond") => "context",
        Some("philosophical") | Some("behavioral") => "context",
        Some("lived-manifestation") | Some("lived-instance") | Some("theoretical-frame") => {
            "context"
        }
        Some("design") | Some("design-conversation") => "context",
        Some("working-thought") | Some("working-cognition") => "context",
        Some("schema-foundation") => "context",
        Some("originating-tension") | Some("fine-tuning-seed") | Some("bt-grounding") => {
            "context"
        }
        Some("origin-tension") | Some("reframing-cognition") => "context",
        Some("three-layer-connection") => "context",

        // Continuation family: sequential progression
        Some("continuation") | Some("continues") => "continuation",
        Some("next-step") | Some("next-phase") => "continuation",
        Some("recordref-evolution") => "continuation",

        // Contrast family: instructive differences
        Some("tensioned") => "contrast",

        // Reference family: related without stronger claim
        Some("echoes") | Some("echoes-projection-extraction") => "reference",
        Some("duplicate") => "reference",
        Some("assessment") | Some("connection-insight") | Some("evidence") => "reference",
        Some("naming-decision") | Some("outcome-formatting") => "reference",
        Some("handoff") | Some("summary") | Some("observation") | Some("reflection") => {
            "reference"
        }
        Some("continuity-milestone") | Some("readiness") => "reference",
        Some("reconstruction") | Some("recovered-decision") | Some("recovered-ordering") => {
            "reference"
        }
        Some("cli-surface") | Some("cli-surface-completion") => "reference",
        Some("input-symmetry") | Some("display-question") => "reference",
        Some("dream-size-connection") | Some("versioning-connection") => "reference",
        Some("enables-dream-synthesis") | Some("migration-reframing") => "reference",
        Some("plan-verification") | Some("slice-3-completion") => "reference",
        Some("gap-discovered") | Some("consolidation-watch-point") => "reference",
        Some("replay-as-deterministic-stack") | Some("replay-reframing") => "reference",
        Some("priority-3-framing") | Some("session-41-opening") => "reference",

        // Wildcard patterns for compound roles
        Some(role) if role.starts_with("structural-answer-") => "reference",
        Some(role) if role.starts_with("first-in-flow-") => "reference",
        Some(role) if role.starts_with("construction-vs-") => "reference",
        Some(role) if role.starts_with("builds-on") => "context",
        Some(role) if role.starts_with("link-columns-") => "context",
        Some(role) if role.starts_with("by-link-") => "context",
        Some(role) if role.starts_with("routing-") => "context",

        // Default: no role or unrecognized role
        None => "reference",
        Some(other) => {
            eprintln!("  warning: unmapped role '{other}', defaulting to 'reference'");
            "reference"
        }
    }
}

/// Read experience refs directly via raw SQL, since Database doesn't expose
/// its inner connection. We open a second read-only connection for this.
fn read_experience_refs(
    db_path: &str,
) -> Result<Vec<(String, Option<String>, Option<String>)>, Box<dyn std::error::Error>> {
    let conn = rusqlite::Connection::open_with_flags(
        db_path,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    )?;

    let mut stmt = conn.prepare(
        "select experience_id, entity_ref, role \
         from experience_ref order by rowid",
    )?;

    let rows: Vec<(String, Option<String>, Option<String>)> = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Option<String>>(1)?,
                row.get::<_, Option<String>>(2)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(rows)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: migrate-experience-refs <brain-db-path>");
        std::process::exit(1);
    }

    let db_path = &args[1];
    println!("Opening brain database: {db_path}");

    // Read refs via a separate read-only connection
    let rows = read_experience_refs(db_path)?;
    let total = rows.len();
    println!("Found {total} experience refs");

    // Open the brain DB for writing events
    let db = Database::open_brain(db_path)?;

    let mut converted = 0u32;
    let mut skipped = 0u32;

    for (experience_id, entity_ref_str, role) in &rows {
        // Skip NULL entity_ref (legacy pre-Ref data)
        let Some(entity_ref_str) = entity_ref_str else {
            skipped += 1;
            continue;
        };

        // Parse the entity ref — try JSON first, then RefToken
        let to_ref = match serde_json::from_str::<Ref>(entity_ref_str) {
            Ok(r) => r,
            Err(_) => match entity_ref_str.parse::<RefToken>() {
                Ok(token) => token.into_inner(),
                Err(e) => {
                    eprintln!(
                        "  warning: skipping unparseable ref for experience {experience_id}: {e}"
                    );
                    skipped += 1;
                    continue;
                }
            },
        };

        // Build from_ref pointing to the experience
        let exp_id: ExperienceId = experience_id.parse()?;
        let from_ref = Ref::experience(exp_id);

        // Map role to nature
        let nature = map_role_to_nature(role.as_deref());

        // Create connection and log the event
        let connection = Connection::create(NatureName::new(nature), from_ref, to_ref);
        let event = Events::Connection(ConnectionEvents::ConnectionCreated(connection));
        db.log_event(&event, projections::brain::ALL)?;

        converted += 1;
    }

    println!("\nMigration complete:");
    println!("  Total refs:  {total}");
    println!("  Converted:   {converted}");
    println!("  Skipped:     {skipped}");

    Ok(())
}
