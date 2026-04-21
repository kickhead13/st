use std::process::Command;

fn run_scp_command(args: Vec<&str>) -> std::io::Result<()> {

    let status = Command::new("scp")
        .args(args)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "scp failed"))
    }
}

fn clone_st_dir(ssh_uri: &str, local_dir: &str) -> std::io::Result<()> {

    let remote_st = format!("{}/.st", ssh_uri);

    let flags = vec!["-r", &remote_st, local_dir]; 

    if let Ok(extra_flags) = std::env::var("ST_SSH_FLAGS") {
        let extra_flags_vec: Vec<&str> = extra_flags.split_whitespace().collect();
        let mut all_flags = flags.clone();
        all_flags.splice(0..0, extra_flags_vec);
        return run_scp_command(all_flags);
    } else {
        return run_scp_command(flags);
    }
}

fn main() {
    let remote_uri = match std::fs::read_to_string(".st/REMOTE") {
        Ok(uri) => uri.trim().to_string(),
        Err(e) => {
            eprintln!("Failed to read remote URI from .st/REMOTE. Make sure you have initialized the repository and set a remote URI. More details:\n{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = clone_st_dir(&remote_uri, ".") {
        eprintln!("Failed to clone .st directory from remote. More details:\n{}", e);
        std::process::exit(1);
    }
}
