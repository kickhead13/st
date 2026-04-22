use clap::{Parser};
use std::fs;
use std::io::{self, BufRead};

fn check_labels(args: &Args, labels_path: &str) -> bool {
    if let Some(labels) = &args.labels {
        let labels_arr = labels.split(',');
        if let Ok(file) = fs::File::open(&labels_path) {
            for line in io::BufReader::new(file).lines().flatten() {
                for label in labels_arr.clone() {
                    if label == line {
                        return true;
                    }
                }
            }
        }
    } else {
        return true;
    }
    return false;
}

fn list_tasks(args: Args, task_path: &str, task: &str) {
    if args.verbose {
        let labels_path = format!("{}/LABELS", task_path);
        
        if ! check_labels(&args, &labels_path) {
            return;
        }

        println!("{}", task);
        if let Ok(file) = fs::File::open(&labels_path) {
            println!("  Labels:");
            for line in io::BufReader::new(file).lines().flatten() {
                println!("    {}", line);
            }
        }
        let desc_path = format!("{}/DESC", task_path);
        if let Ok(desc) = fs::read_to_string(&desc_path) {
            println!("  Description:\n    {}", desc);
        }

    } else if args.notes {
        let notes_path = format!("{}/NOTES", task_path);

        if ! check_labels(&args, &format!("{}/LABELS", task_path)) {
            return;
        }
        if let Ok(file) = fs::File::open(&notes_path) {
            println!("  Notes:");
            for line in io::BufReader::new(file).lines().flatten() {
                println!("    {}", line);
            }
        }
    } else {
        if ! check_labels(&args, &format!("{}/LABELS", task_path)) {
            return;
        }
        println!("{}", task);
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
    #[arg(short, long)]
    verbose: bool,

    /// Show notes
    #[arg(short = 'n', long)]
    notes: bool,

    /// Filter listing by comma separated labels. (LABEL1=l1,LABEL2=l2)
    #[arg(short, long)]
    labels: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let topics_path = ".st/topics";
    if let Some(topic) = &args.topic {
        let topic_path = format!("{}/{}", topics_path, topic);
        if let Some(task) = &args.task.clone() {
            let task_path = format!("{}/{}", topic_path, task);
            list_tasks(args, &task_path, task);
        } else {
            if let Ok(entries) = fs::read_dir(&topic_path) {
                for entry in entries.flatten() {
                    if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                        let task_name = entry.file_name().to_string_lossy().to_string();
                        let task_entry_path = entry.path();
                        let task_path = task_entry_path.to_str().unwrap_or("Could not unwrap path to task.");
                        
                        list_tasks(args.clone(),task_path, &task_name);
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
