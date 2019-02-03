use clap::{App, Arg};
use json::*;
use std::io::Error;
use std::result::Result;
use std::string::String;

mod input;
mod selection;

fn match_line(filter: &str, line: Result<String, Error>) {
    let input = line.expect("Could not read line from standard in");
    let json_input = json::parse(&input);
    if json_input.is_ok() {
        match_json(filter, json_input.unwrap())
    }
}

fn match_json(filter: &str, json_input: JsonValue) {
    let selection_matches = selection::identity::greedily_matches(Some(filter));
    if selection_matches.is_ok() && selection::identity::identity(Some(&json_input)).is_some() {
        println!("{}", json::stringify(json_input))
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
        input::print_input_file(filter, in_file, &match_line);
    } else {
        input::print_input(filter, &match_line);
    }
}
