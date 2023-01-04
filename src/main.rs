use clap::{App,Arg};

fn main() {
    let args = App::new("rlox")
        .arg(Arg::with_name("filepath")
             .takes_value(true))
        .get_matches();
    
    match args.value_of("filepath") {
        Some(filepath) => {
            run_file(filepath);
        },
        None => {
            run_prompt();
        }
    }
}

fn run_file(filepath: &str) {
    print!("{:}", filepath);
}

fn run_prompt() {
    print!("runPrompt()");
}
