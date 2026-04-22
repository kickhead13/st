use clap::Parser;
use std::process::Command;
use std::fs;
use std::io::{self};

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

    /// Add a note (opens $EDITOR)
    #[arg(long,short)]
    note: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if let Some(topic) = &args.topic {
        if args.add.is_none() && args.task.is_none() && !args.desc && !args.note {
            // Create topic
            fs::create_dir_all(format!(".st/topics/{}", topic))?;
            return Ok(());
        }
        if let Some(task) = &args.add {
            // Create task
            fs::create_dir_all(format!(".st/topics/{}/{}", topic, task))?;
            return Ok(());
        }
        if let (Some(task), true) = (&args.task, args.desc) {
            // Add description using $EDITOR
            let desc_path = format!(".st/topics/{}/{}/description.txt", topic, task);
            fs::create_dir_all(format!(".st/topics/{}/{}", topic, task))?;
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
            Command::new(editor)
                .arg(&desc_path)
                .status()
                .expect("Failed to open editor");
            return Ok(());
        }
        if let (Some(task), true) = (&args.task, args.note) {
            // Add note using $EDITOR
            let notes_path = format!(".st/topics/{}/{}/notes.txt", topic, task);
            fs::create_dir_all(format!(".st/topics/{}/{}", topic, task))?;
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
            Command::new(editor)
                .arg(&notes_path)
                .status()
                .expect("Failed to open editor");
            return Ok(());
        }
    }
    eprintln!("Invalid argument combination.");
    Ok(())
}
