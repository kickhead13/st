use clap::{Parser, ArgAction};
use std::fs;
use std::io::{self, BufRead};


fn list_tasks(args: Args, task_path: &str) {
    if args.verbose {
        let desc_path = format!("{}/description.txt", task_path);
        if let Ok(desc) = fs::read_to_string(&desc_path) {
            println!("  Description:\n    {}", desc);
        }
    }
    if args.notes {
        let notes_path = format!("{}/notes.txt", task_path);
        if let Ok(file) = fs::File::open(&notes_path) {
            println!("  Notes:");
            for line in io::BufReader::new(file).lines().flatten() {
                println!("    {}", line);
            }
        }
    }
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
struct Args {
    /// List all tasks in all topics.
    #[arg(short, long)]
    all: bool,

    /// Topic to list or inspect
    #[arg(short = 'T', long)]
    topic: Option<String>,

    /// Task to list or inspect (must be used with --topic)
    #[arg(short = 't', long)]
    task: Option<String>,

    /// Show descriptions
    #[arg(short, long, action = ArgAction::SetTrue)]
    verbose: bool,

    /// Show notes
    #[arg(short = 'n', long, action = ArgAction::SetTrue)]
    notes: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let topics_path = ".st/topics";
    if let Some(topic) = &args.topic {
        let topic_path = format!("{}/{}", topics_path, topic);
        if let Some(task) = &args.task {
            println!("{}", task);
            let task_path = format!("{}/{}", topic_path, task);
            list_tasks(args, &task_path);
        } else {
            if let Ok(entries) = fs::read_dir(&topic_path) {
                for entry in entries.flatten() {
                    if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                        let task_name = entry.file_name().to_string_lossy().to_string();
                        println!("{}", task_name);
                        let task_entry_path = entry.path();
                        let task_path = task_entry_path.to_str().unwrap_or("Could not unwrap path to task.");

                        list_tasks(args.clone(),task_path);
                        // println!();
                    }
                }
            }
        }
    } else {
        // List all topics
        if let Ok(entries) = fs::read_dir(topics_path) {
            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    println!("{}", entry.file_name().to_string_lossy());
                }
            }
        }
    }
    Ok(())
}
