use clap::{App,Arg};
use std::fs;
use std::io;

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
    let contents = fs::read_to_string(filepath)
        .expect(&format!("Failed to read from given filepath: {:?}", filepath));
    println!("Contents: {contents}");
}

fn run_prompt() {
    let lines = io::stdin().lines();
    for line in lines {
        println!("got a line: {}", line.unwrap());
    }
}
