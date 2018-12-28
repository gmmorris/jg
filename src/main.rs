use clap::{Arg, App};
use std::io::{self, BufRead};

mod filtering;

fn print_input(filter: &str) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if filtering::identity::matches(filter) {
            let line = line.expect("Could not read line from standard in");
            println!(" - {}", line);
        }
    }
}

fn main() {
    let matches = App::new("jgrep")
        .version("0.0.1")
        .author("Gidi Meir Morris <gidi@gidi.io>")
        .about("jgrep searches for PATTERNS in json input, jgrep prints each json object that matches a pattern.")
        .arg(
            Arg::with_name("filter")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("JSON query filter")
        )
        .get_matches();
    
    let filter = matches.value_of("filter").unwrap();
    
    println!("filter: {}", filter);

    print_input(filter);
}