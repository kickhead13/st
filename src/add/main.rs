use clap::Parser;
use std::process::Command;
use std::fs;
use std::io::{self};

fn open_editor(topic: &str, task: &str, path: &str) -> io::Result<()> {
    fs::create_dir_all(format!(".st/topics/{}/{}", topic, task))?;
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
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if let Some(topic) = &args.topic {
        if args.add.is_none() && args.task.is_none() && !args.desc && !args.note {
            fs::create_dir_all(format!(".st/topics/{}", topic))?;
            return Ok(());
        }
        if let Some(task) = &args.add {
            fs::create_dir_all(format!(".st/topics/{}/{}", topic, task))?;
            return Ok(());
        }
        if let (Some(task), true) = (&args.task, args.desc) {
            let desc_path = format!(".st/topics/{}/{}/DESC", topic, task);
            open_editor(topic, task, &desc_path)?;
        }
        if let (Some(task), true) = (&args.task, args.note) {
            let notes_path = format!(".st/topics/{}/{}/NOTES", topic, task);
            open_editor(topic, task, &notes_path)?;
        }
        if let (Some(task), true) = (&args.task, args.labels) {
            let desc_path = format!(".st/topics/{}/{}/LABELS", topic, task);
            open_editor(topic, task, &desc_path)?;
        }

    }
    eprintln!("Invalid argument combination.");
    Ok(())
}
