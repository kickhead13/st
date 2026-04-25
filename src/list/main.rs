use clap::{Parser};
use std::fs;
use std::io::{self, BufRead};
use std::os::unix::io::AsRawFd;
fn check_labels(args: &Args, labels_path: &str) -> bool {
    if let Some(labels) = &args.labels {
        let labels_arr = labels.split(',');
        if let Ok(file) = fs::File::open(labels_path) {
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
    false
}

fn list_task_markdown(args: &Args, task_path: &str, task: &str) {
    let mut output = format!("\n# {}", task);

    if let Ok(file) = fs::File::open(format!("{}/SHORT_DESC.md", task_path)) 
    && let Some(first_line) = io::BufReader::new(file).lines().flatten().next() 
    && !args.state_md {
        output.push_str(&format!(" ({})", first_line));
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

/// Lists tasks and topics. Can be used to print out information in either hierarchical or markdown
/// format.
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

    /// Outputs to STATE.md of the specified topic iff --topic and --markdown are set.
    /// Useful if you want to edit the contents of a topic in bulk (in the generated STATE.md) and then apply it with `st apply`.
    #[arg(short='S', long)]
    state_md: bool,
}

fn main() -> io::Result<()> {
    let mut args = Args::parse();

    if let Ok(st_toml) = fs::read_to_string("st.toml") && !args.bypass_config 
    && let Ok(parsed) = st_toml.parse::<toml::Table>() 
    && let Some(active) = parsed.get("active").and_then(|v| v.as_str()) {
        let active_labels = active.split(',').map(|s| s.trim()).collect::<Vec<&str>>().join(",");
        if let Some(existing_labels) = &args.labels {
            let combined_labels = format!("{},{}", existing_labels, active_labels);
            args.labels = Some(combined_labels);
        } else {
            args.labels = Some(active_labels);
        }
    }

    let topics_path = "st/topics";
    if let Some(topic) = &args.topic {

        if ! args.markdown && args.state_md {
            eprintln!("Error: --state-md can only be used with --markdown.");
            std::process::exit(1);
        }

        if args.state_md {
            let state_md_path = format!("{}/{}/STATE.md", topics_path, topic);
            let file = fs::File::create(&state_md_path)?;

            let fd = file.as_raw_fd();
    
            unsafe {
                // TODO: figure out if this can be done without unsage code...
                libc::dup2(fd, 1);
            }
        }

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
