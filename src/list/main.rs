use clap::{Parser};
use std::fs;
use std::io::{self, BufRead};
use toml;

fn check_labels(args: &Args, labels_path: &str) -> bool {
    if let Some(labels) = &args.labels {
        let labels_arr = labels.split(',');
        if let Ok(file) = fs::File::open(&labels_path) {
            let lines = io::BufReader::new(file).lines().flatten().collect::<Vec<String>>();
            for label in labels_arr.clone() {
                if ! lines.contains(&label.to_string()) {
                    return false;
                }
            }
            return true;
        }
    } else {
        return true;
    }
    return false;
}

fn list_task_markdown(args: &Args, task_path: &str, task: &str) {
    let mut output = format!("# {}", task);

    if let Ok(file) = fs::File::open(format!("{}/SHORT_DESC.md", task_path)) {
        if let Some(first_line) = io::BufReader::new(file).lines().flatten().next() {
            output.push_str(&format!(" ({})", first_line));
        }
    }

    println!("{}", output);

    if args.verbose {
        let labels_path = format!("{}/LABELS", task_path);
        
        if let Ok(file) = fs::File::open(&labels_path) {
            println!("**Labels:**");
            for line in io::BufReader::new(file).lines().flatten() {
                println!("- {}", line);
            }
        }
        let desc_path = format!("{}/DESC.md", task_path);
        if let Ok(desc) = fs::read_to_string(&desc_path) {
            println!("## Description:\n{}", desc);
        }

    }
    
    if args.notes {
        let notes_path = format!("{}/NOTES.md", task_path);

        if let Ok(file) = fs::File::open(&notes_path) {
            println!("## Notes:");
            for line in io::BufReader::new(file).lines().flatten() {
                println!("{}", line);
            }
        }
    }
}

fn list_task(args: Args, task_path: &str, task: &str) {

    if ! check_labels(&args, &format!("{}/LABELS", task_path)) {
        return;
    }

    if args.markdown {
        list_task_markdown(&args, task_path, task);
        return;
    }

    if let Ok(file) = fs::File::open(format!("{}/SHORT_DESC.md", task_path)) {
        if let Some(first_line) = io::BufReader::new(file).lines().flatten().next() {
            println!("{} ({})", task, first_line);
        } else {
            return;
        }
    } else {
        println!("{}", task);
    }

    if args.verbose {
        let labels_path = format!("{}/LABELS", task_path);
        
        if let Ok(file) = fs::File::open(&labels_path) {
            println!("  Labels:");
            for line in io::BufReader::new(file).lines().flatten() {
                println!("    {}", line);
            }
        }
        let desc_path = format!("{}/DESC.md", task_path);
        if let Ok(desc) = fs::read_to_string(&desc_path) {
            println!("  Description:\n    {}", desc);
        }

    }
    
    if args.notes {
        let notes_path = format!("{}/NOTES.md", task_path);

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
    /// Bypass config.
    #[arg(short, long)]
    bypass_config: bool,

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

    /// Format output as Markdown.
    #[arg(short='M', long)]
    markdown: bool,
}

fn main() -> io::Result<()> {
    let mut args = Args::parse();

    if let Ok(st_toml) = fs::read_to_string("st.toml") && !args.bypass_config {
        if let Ok(parsed) = st_toml.parse::<toml::Table>() {
            if let Some(active) = parsed.get("active").and_then(|v| v.as_str()) {
                let active_labels = active.split(',').map(|s| s.trim()).collect::<Vec<&str>>().join(",");
                if let Some(existing_labels) = &args.labels {
                    let combined_labels = format!("{},{}", existing_labels, active_labels);
                    args.labels = Some(combined_labels);
                } else {
                    args.labels = Some(active_labels);
                }
            }
        }   
    }

    let topics_path = "st/topics";
    if let Some(topic) = &args.topic {
        let topic_path = format!("{}/{}", topics_path, topic);
        if let Some(task) = &args.task.clone() {
            let task_path = format!("{}/{}", topic_path, task);
            list_task(args, &task_path, task);
        } else {
            if let Ok(entries) = fs::read_dir(&topic_path) {
                for entry in entries.flatten() {
                    if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                        let task_name = entry.file_name().to_string_lossy().to_string();
                        let task_entry_path = entry.path();
                        let task_path = task_entry_path.to_str().unwrap_or("Could not unwrap path to task.");
                        
                        list_task(args.clone(),task_path, &task_name);
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
