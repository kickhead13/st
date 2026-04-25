use clap::Parser;

/// Initializes a new shtrack local/remote repository.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Creates an empty repository and sets the remote to the given URI. It is still required to
    /// manually pull the remote repository to populate the local instance with tasks.
    #[arg(short, long)]
    remote: Option<String>,
    
    /// Creates a bare "repository". It is useful for creating new remote repositories.
    #[arg(short, long)]
    bare: bool,
}

fn create_repository() {
    if let Err(e) = std::fs::create_dir_all("st/topics") {
        eprintln!("Failed to create st directory. This could be due to it already existing or maybe not enough permissions. More details:\n{}", e);
        std::process::exit(1);
    }


    if let Err(e) = std::fs::create_dir_all("st/templates/task") {
        eprintln!("Failed to create st/templates directory. This could be due to it already existing or maybe not enough permissions. More details:\n{}", e);
        std::process::exit(1);
    }
}
fn main() {
    let args = Args::parse();

    if let Some(remote) = args.remote {
        if !args.bare {
            create_repository();
            std::fs::write("st/REMOTE", remote).expect("Failed to write remote URI");
            std::process::exit(0);
        } else if args.bare {
            eprintln!("Cannot set a remote URI for a bare repository.");
            std::process::exit(1);
        }
    }
    create_repository();

}
