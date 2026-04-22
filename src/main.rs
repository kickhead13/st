static HELP_MESSAGE: &str = r#"Usage: shtrack <PROGRAM>
Keep track of notes and things that you need to do, from the
comfort of your shell.

PROGRAM:
    init    initialize the .st repository
    add     add new topics, tasks, descriptions and notes
    list    list topics, tasks, descriptions and notes
    pull    pull from a remote repository
    push    push to a remote repository
"#;

fn main() {
    let cli_args: Vec<String> = std::env::args().collect();
    if cli_args.len() < 2 {
        print!("{HELP_MESSAGE}");
        std::process::exit(0);
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
