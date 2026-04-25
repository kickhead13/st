static HELP_MESSAGE: &str = r#"Usage: st <PROGRAM>
Track your tasks... in your Shell.

PROGRAM:
    init    initialize the st repository
    add     add new topics, tasks, descriptions and notes
    apply   apply all topics' STATE.md files
    list    list topics, tasks, descriptions and notes
    pull    pull from a remote repository
    push    push to a remote repository (not implemented yet)
    rm      remove topics, tasks, descriptions and notes (not implemented yet)
    server  start a markdown server at localhost:9090 use localhost:9090/list/{topic}/{labels}
"#;

fn main() {
    let cli_args: Vec<String> = std::env::args().collect();
    if cli_args.len() < 2 {
        print!("{HELP_MESSAGE}");
        std::process::exit(0);
    }

    if let Ok(st_toml) = std::fs::read_to_string("st.toml") 
    && let Ok(parsed) = st_toml.parse::<toml::Table>()
    && let Some(apply) = parsed.get("apply").and_then(|v| v.as_str())  
    && (apply == "always" || apply == "always&delete") 
    && let Err(e) = std::process::Command::new("st").args(["apply"]).status() {
        eprintln!("{}", e);
        print!("{HELP_MESSAGE}");
        std::process::exit(-1);
    }

    let program = format!("st-{}", cli_args[1]);
    match std::process::Command::new(program).args(&cli_args[2..]).status() {
        Err(e) => {
            eprintln!("{}", e);
            print!("{HELP_MESSAGE}");
            std::process::exit(-1);
        },
        Ok(exit_code) => {
            if let Some(code) = exit_code.code() {
                std::process::exit(code);
            }
        }
    }
    
}
