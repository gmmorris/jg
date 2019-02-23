#[macro_use]
extern crate lazy_static;
extern crate regex;

use clap::{App, Arg};

mod input;
mod selection;

fn main() {
    let matches = App::new("jgrep")
        .version("0.0.1")
        .author("Gidi Meir Morris <gidi@gidi.io>")
        .about("jgrep searches for PATTERNS in json input, jgrep prints each json object that matches a pattern.")
        .arg(
            Arg::with_name("filter")
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
            Arg::with_name("count")
                .short("c")
                .help("JSON input file")
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .help("Sets the level of verbosity")
        )
        .get_matches();

    let filter = matches.value_of("filter").unwrap_or(".");

    let config = input::Config {
        print_only_count: matches.is_present("count"),
    };

    let matched_filters = selection::match_filters(filter);
    let count = input::match_input(matches.value_of("input"), &|line| match input::match_line(
        &matched_filters,
        &config,
        line,
    ) {
        Ok(matched_line) => {
            if !config.print_only_count {
                println!("{}", matched_line);
            }
            Ok(())
        }
        Err(_) => Err(()),
    });

    if config.print_only_count {
        println!("{}", count);
    }
}
