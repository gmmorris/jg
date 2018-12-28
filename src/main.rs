use clap::{Arg, App};
use std::io::{self, BufRead, BufReader, Error};
use std::string::String;
use std::result::Result;
use std::fs::File;

mod selection;

fn print_input(filter: &str) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match_line(filter, line)
    }
}

fn print_input_file(filter: &str, input: &str) {
    let file = File::open(input).unwrap();
    for line in BufReader::new(file).lines() {
        match_line(filter, line)
    }
}

fn match_line(filter: &str, line: Result<String, Error>) {
    if selection::identity::matches(filter) {
        let line = line.expect("Could not read line from standard in");
        println!("{}", line);
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
                .multiple(true)
                .help("JSON query filter")
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .takes_value(true)
                .help("JSON input file")
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .help("Sets the level of verbosity")
        )
        .get_matches();
    
    let filter = matches.value_of("filter").unwrap();
    
    if matches.is_present("v") {
        verbose(filter);
    }

    if let Some(in_file) = matches.value_of("input") {
        print_input_file(filter, in_file);
    } else {
        print_input(filter);
    }
}