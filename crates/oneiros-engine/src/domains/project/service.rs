use std::io::BufRead;
use std::path::Path;

use crate::event::repo;
use crate::*;

pub struct ProjectService;

impl ProjectService {
    pub fn init(
        ctx: &SystemContext,
        brain_name: String,
    ) -> Result<ProjectResponse, Box<dyn std::error::Error>> {
        let brain_name_typed = BrainName::new(&brain_name);

        if let Ok(BrainResponse::Found(_)) = BrainService::get(ctx, &brain_name_typed) {
            return Ok(ProjectResponse::BrainAlreadyExists(brain_name_typed));
        }

        BrainService::create(ctx, BrainName::new(&brain_name))?;

        let actors = ctx
            .with_db(|conn| ActorRepo::new(conn).list())
            .map_err(|e| format!("database error: {e}"))?;

        if let Some(actor) = actors.first() {
            TicketService::create(ctx, actor.id.clone(), BrainName::new(&brain_name))?;
        }

        Ok(ProjectResponse::BrainCreated(BrainName::new(&brain_name)))
    }

    /// Export all events to a JSONL file in the target directory.
    pub fn export(
        ctx: &ProjectContext,
        target_dir: &Path,
        project_name: &str,
    ) -> Result<ProjectResponse, Box<dyn std::error::Error>> {
        let events = ctx.with_db(repo::load_events)?;

        let mut buffer = String::new();
        for event in &events {
            let export: ExportEvent = event.clone().into();
            buffer.push_str(&serde_json::to_string(&export)?);
            buffer.push('\n');
        }

        std::fs::create_dir_all(target_dir)?;

        let date = chrono::Utc::now().format("%Y-%m-%d");
        let file_name = format!("{project_name}-{date}-export.jsonl");
        let file_path = target_dir.join(file_name);

        std::fs::write(&file_path, buffer)?;

        Ok(ProjectResponse::WroteExport(file_path))
    }

    /// Import events from a JSONL file and replay projections.
    pub fn import(
        ctx: &ProjectContext,
        file_path: &Path,
    ) -> Result<ProjectResponse, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        let mut imported = 0;

        ctx.with_db(|conn| -> Result<(), Box<dyn std::error::Error>> {
            for line in reader.lines() {
                let line = line?;
                if line.trim().is_empty() {
                    continue;
                }

                let event: ImportEvent = serde_json::from_str(&line)?;
                let event = event.with_source(Source::default());
                repo::import_event(conn, &event)?;
                imported += 1;
            }
            Ok(())
        })?;

        let replayed = ctx.replay()?;

        Ok(ProjectResponse::Imported(ImportResult {
            imported,
            replayed,
        }))
    }

    /// Replay all events through projections, rebuilding read models.
    pub fn replay(ctx: &ProjectContext) -> Result<ProjectResponse, Box<dyn std::error::Error>> {
        let replayed = ctx.replay()?;
        Ok(ProjectResponse::Replayed(ReplayResult { replayed }))
    }
}
