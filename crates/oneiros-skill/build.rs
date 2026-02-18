use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=skill");
    println!("cargo:rerun-if-changed=../../Cargo.toml");

    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = Path::new(&manifest_dir)
        .join("../..")
        .canonicalize()
        .unwrap();
    let source = Path::new(&manifest_dir).join("skill");
    let dist_dir = workspace_root.join("dist");

    // Write marketplace.json
    let marketplace = fs::read_to_string(source.join("marketplace.json")).unwrap();
    let plugins_dir = workspace_root.join(".claude-plugin/marketplace.json");
    write_stamped(&plugins_dir, &marketplace, &version);

    // Write SKILL.md
    let skill_md = fs::read_to_string(source.join("SKILL.md")).unwrap();
    let dest = dist_dir.join("skills/oneiros/SKILL.md");
    write_stamped(&dest, &skill_md, &version);

    // Write plugin.json
    let plugin = fs::read_to_string(source.join("plugin.json")).unwrap();
    let dest = dist_dir.join(".claude-plugin/plugin.json");
    write_stamped(&dest, &plugin, &version);

    // Write hooks.json
    let hooks = fs::read_to_string(source.join("hooks.json")).unwrap();
    let dest = dist_dir.join("hooks/hooks.json");
    write_file(&dest, &hooks);

    // Write command files
    let commands_dir = source.join("commands");
    if commands_dir.exists() {
        for entry in fs::read_dir(&commands_dir).unwrap() {
            let entry = entry.unwrap();
            let name = entry.file_name();
            let content = fs::read_to_string(entry.path()).unwrap();
            let dest = dist_dir.join("commands").join(&name);
            write_file(&dest, &content);
        }
    }

    // Write agent definition files
    let agents_dir = source.join("agents");
    if agents_dir.exists() {
        for entry in fs::read_dir(&agents_dir).unwrap() {
            let entry = entry.unwrap();
            let name = entry.file_name();
            let content = fs::read_to_string(entry.path()).unwrap();
            let dest = dist_dir.join("agents").join(&name);
            write_file(&dest, &content);
        }
    }

    // Write AGENTS.md template
    let agents_md = fs::read_to_string(source.join("agents-md.md")).unwrap();
    let dest = dist_dir.join("agents-md.md");
    write_file(&dest, &agents_md);

    // Write resource files
    let resources_dir = source.join("resources");
    if resources_dir.exists() {
        for entry in fs::read_dir(&resources_dir).unwrap() {
            let entry = entry.unwrap();
            let name = entry.file_name();
            let content = fs::read_to_string(entry.path()).unwrap();
            let dest = dist_dir.join("skills/oneiros/resources").join(&name);
            write_file(&dest, &content);
        }
    }
}

fn write_stamped(path: &Path, content: &str, version: &str) {
    let stamped = content.replace("{{VERSION}}", version);
    write_file(path, &stamped);
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}
