use clap::Parser;
use std::process::Command;
use std::fs;
use std::io::{self};
use std::path::Path;

fn open_editor(args: &Args, topic: &str, task: &str, path: &str) -> io::Result<()> {
    fs::create_dir_all(format!(".st/topics/{}/{}", topic, task))?;

    if args.commit {
        let git_log = Command::new("git")
            .args(["log", "-1", "--pretty=format:%s (%h) by %aN<%ae> on %aD"])
            .output()
            .expect("Failed to execute git command");
        let git_info = String::from_utf8_lossy(&git_log.stdout);
        let content = fs::read_to_string(path).unwrap_or_default();
        if content.ends_with('\n') {
            fs::write(path, format!("{}### {}", content, git_info))?;
        } else {
            fs::write(path, format!("{}\n### {}", content, git_info))?;
        }
    }
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    Command::new(editor)
        .arg(&path)
        .status()
        .expect("Failed to open editor");
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Topic name
    #[arg(long,short='T')]
    topic: Option<String>,

    /// Task name
    #[arg(long,short)]
    task: Option<String>,

    /// Add a new task
    #[arg(long,short)]
    add: Option<String>,

    /// Add a description to a task (opens $EDITOR)
    #[arg(long,short)]
    desc: bool,

    /// Add labels to a task (opens $EDITOR)
    #[arg(long,short)]
    labels: bool,

    /// Add a note (opens $EDITOR)
    #[arg(long,short)]
    note: bool,

    /// Should be used along side either one of -d, -l or -n to append git commit details before opening the editor.
    #[arg(long,short)]
    commit: bool,

    /// Add a short (one line) description to a task.
    #[arg(long,short='s')]
    short_desc: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if let Some(topic) = &args.topic {
        if args.add.is_none() && args.task.is_none() && !args.desc && !args.note {
            fs::create_dir_all(format!(".st/topics/{}", topic))?;
            return Ok(());
        }
        if let Some(task) = &args.add {
            let template_path = if Path::new(&format!(".st/templates/task.{}", topic)).exists() {
                format!(".st/templates/task.{}", topic)
            } else {
                ".st/templates/task".to_string()
            };
            let task_path = format!(".st/topics/{}/{}", topic, task);
            fs::create_dir_all(&task_path)?;
            for entry in fs::read_dir(template_path)? {
                let entry = entry?;
                let file_type = entry.file_type()?;
                if file_type.is_file() {
                    let file_name = entry.file_name();
                    fs::copy(entry.path(), format!("{}/{}", task_path, file_name.to_string_lossy().to_string()))?;
                }
            }
            return Ok(());
        }
        if let (Some(task), true) = (&args.task, args.desc) {
            let desc_path = format!(".st/topics/{}/{}/DESC.md", topic, task);
            open_editor(&args, topic, task, &desc_path)?;
        }
        if let (Some(task), true) = (&args.task, args.short_desc) {
            let desc_path = format!(".st/topics/{}/{}/SHORT_DESC.md", topic, task);
            open_editor(&args, topic, task, &desc_path)?;
        }
        if let (Some(task), true) = (&args.task, args.note) {
            let notes_path = format!(".st/topics/{}/{}/NOTES.md", topic, task);
            open_editor(&args, topic, task, &notes_path)?;
        }
        if let (Some(task), true) = (&args.task, args.labels) {
            let desc_path = format!(".st/topics/{}/{}/LABELS", topic, task);
            open_editor(&args, topic, task, &desc_path)?;
        }
        return Ok(());
    }
    eprintln!("Invalid argument combination.");
    Ok(())
}
