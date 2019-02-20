#[macro_use]
extern crate lazy_static;
extern crate regex;

use clap::{App, Arg};

mod input;
mod selection;


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

    input::match_input(
        matches.value_of("input"),
        selection::match_filters(filter),
        &input::match_line,
    );
}
