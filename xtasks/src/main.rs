use std::path::PathBuf;

use clap::{Parser, Subcommand};
use oneiros_engine::SkillPackage;

#[derive(Debug, Parser)]
#[command(name = "xtask")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Build the Claude Code plugin — emit skill assets to dist/ and
    /// marketplace manifest to .claude-plugin/.
    PluginBuild,
}

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::PluginBuild => plugin_build()?,
    }

    Ok(())
}

fn plugin_build() -> Result<(), Box<dyn core::error::Error>> {
    let workspace_root = workspace_root()?;
    let dist = workspace_root.join("dist");
    let claude_plugin = workspace_root.join(".claude-plugin");

    // Clean previous build artifacts for a known-good state.
    if dist.exists() {
        std::fs::remove_dir_all(&dist)?;
    }
    if claude_plugin.exists() {
        std::fs::remove_dir_all(&claude_plugin)?;
    }

    let assets = SkillPackage::assets();
    let mut count = 0;

    for asset in &assets {
        // marketplace.json is the discovery manifest — it lives at the
        // workspace root so Claude Code can find the plugin. Everything
        // else is the plugin payload and goes into dist/.
        let dest = if asset.path == ".claude-plugin/marketplace.json" {
            workspace_root.join(asset.path)
        } else {
            dist.join(asset.path)
        };

        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(&dest, &asset.content)?;
        count += 1;
    }

    println!("Wrote {count} plugin files to {}", dist.display());

    Ok(())
}

/// Discover the workspace root from this crate's manifest directory.
fn workspace_root() -> Result<PathBuf, Box<dyn core::error::Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));

    // xtasks/ is one level below the workspace root
    let root = if manifest_dir.join("../Cargo.toml").exists() {
        manifest_dir.join("..").canonicalize()?
    } else {
        manifest_dir
    };

    Ok(root)
}
