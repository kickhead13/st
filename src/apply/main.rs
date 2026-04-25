use std::fs;
use std::path::Path;
use clap::{Parser};

/// Apply the state from STATE.md files to the task directories, creating LABELS, DESC.md, and NOTES.md files as needed.
/// This command enables users to update an entire topic's task en masse.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
struct Args {
    /// Bypass reading st.toml and applying active labels
    #[arg(long)]
    bypass_config: bool,
}

fn main() {
    let args = Args::parse();
    let mut remove = false;

    if let Ok(st_toml) = fs::read_to_string("st.toml") && !args.bypass_config 
    && let Ok(parsed) = st_toml.parse::<toml::Table>()
    && let Some(apply) = parsed.get("apply").and_then(|v| v.as_str()) {
        if apply == "delete" || apply == "always&delete" {
            remove = true;   
        } else if apply == "never" {
            println!("Applying is disabled by st.toml configuration.");
            return;
        }
    }

    let topics_dir = Path::new("./st/topics");
    for entry in fs::read_dir(topics_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let state_file = path.join("STATE.md");
            if state_file.exists() {
                apply_state(&state_file, &path);
            }
            if remove {
                fs::remove_file(state_file).unwrap();
            }
        }
    }


}

fn apply_state(state_file: &Path, topic_dir: &Path) {
    let content = fs::read_to_string(state_file).unwrap();
    let tasks = content.split("\n# ").filter(|s| !s.trim().is_empty());
    for task in tasks {
        let lines: Vec<&str> = task.lines().collect();
        let task_name = lines[0].trim();
        let task_dir = topic_dir.join(task_name);
        // fs::create_dir_all(&task_dir).unwrap();

        let mut labels = Vec::new();
        let mut description = String::new();
        let mut notes = String::new();
        let mut current_section = "";
        for line in &lines[1..] {
            if line.starts_with("**Labels:**") {
                current_section = "labels";
            } else if line.starts_with("## Description:") {
                current_section = "description";
            } else if line.starts_with("## Notes:") {
                current_section = "notes";
            } else {
                match current_section {
                    "labels" => {
                        if let Some(label) = line.trim().strip_prefix("- ") {
                            labels.push(label.to_string());
                        }
                    }
                    "description" => description.push_str(line),
                    "notes" => notes.push_str(line),
                    _ => {}
                }
            }
        }
        
        fs::write(task_dir.join("LABELS"), labels.join("\n")).unwrap();
        fs::write(task_dir.join("DESC.md"), description).unwrap();
        fs::write(task_dir.join("NOTES.md"), notes).unwrap();
    }
}


