use clap::{Arg, App};
use std::io::{self, BufRead};

mod selection;

fn print_input(filter: &str) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if selection::identity::matches(filter) {
            let line = line.expect("Could not read line from standard in");
            println!("{}", line);
        }
    }
}

fn verbose(filter: &str) {
    println!("filter: {}", filter);
    println!("-----");
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
        .arg(
            Arg::with_name("v")
                .short("v")
                .help("Sets the level of verbosity")
        )
        .get_matches();
    
    let filter = matches.value_of("filter").unwrap();
    
    if matches.occurrences_of("v") > 0 {
        verbose(filter);
    }

    print_input(filter);
}