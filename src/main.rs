use clap::{App, Arg};
use json::*;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, BufRead, BufReader, Error};
use std::result::Result;
use std::string::String;

mod selection;

fn print_input(filter: &str) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match_line(filter, line)
    }
}

fn print_input_file(filter: &str, input: &str) {
    let file = match File::open(input) {
        Ok(contents) => contents,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                panic!("The specified input file could not be found: {:?}", input)
            }
            other_error => panic!(
                "There was a problem opening the file '{:?}': {:?}",
                input, other_error
            ),
        },
    };
    for line in BufReader::new(file).lines() {
        match_line(filter, line)
    }
}

fn match_line(filter: &str, line: Result<String, Error>) {
    let input = line.expect("Could not read line from standard in");
    let json_input = json::parse(&input);
    if json_input.is_ok() {
        match_json(filter, json_input.unwrap())
    }
}

fn match_json(filter: &str, json_input: JsonValue) {
    let selection_matches = selection::identity::greedily_matches(Some(filter));
    match selection_matches {
        Ok(_) => {
            if selection::identity::identity(Some(&json_input)).is_some() {
                println!("{}", json::stringify(json_input))
            }
        }
        Err(unmatchedPattern) => panic!(
            "There was a problem matching the pattern: {:?}",
            unmatchedPattern
        ),
    };
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
