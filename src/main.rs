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
            Arg::with_name("pattern")
                .takes_value(true)
                .help("JSON query filter")
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .help("Only a count of selected lines is written to standard output.")
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .takes_value(true)
                .help("JSON input file")
        )
        .arg(
            Arg::with_name("ignore-case")
                .short("i")
                .long("ignore-case")
                .help("Perform case insensitive matching. By default, **jgrep** is case sensitive.")
        )
        .arg(
            Arg::with_name("max-count")
                .short("m")
                .long("max-count")
                .takes_value(true)
                .help("Stop reading the file after _num_ matches.")
        )
        .arg(
            Arg::with_name("line-number")
                .short("n")
                .long("line-number")
                .help("Each output line is preceded by its relative line number in the file, starting at line 1.")
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .long("silent")
                .help("Quiet mode: suppress normal output.")
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .help("Sets the level of verbosity")
        )
        .get_matches();

    let config = input::Config {
        print_only_count: matches.is_present("count"),
        print_line_number: matches.is_present("line-number"),
        ignore_case: matches.is_present("ignore-case"),
        is_quiet_mode: matches.is_present("quiet"),
        max_num: matches.value_of("max-count").map(|num| {
            usize::from_str_radix(num, 32).expect("an invalid -m/--max-num flag has been specified")
        }),
    };

    let pattern = matches
        .value_of("pattern")
        .and_then(|pattern| {
            input::in_configured_case(pattern, &config).or(Some(String::from(pattern)))
        })
        .unwrap_or(String::from("."));

    let matched_filters = selection::match_filters(&pattern);
    let has_matched = input::match_input(
        matches.value_of("file"),
        &config,
        &|line| input::match_line(&matched_filters, &config, line),
        &|(index, matched_count, matched_result)| {
            if let Ok(matched_line) = matched_result {
                if !(config.print_only_count || config.is_quiet_mode) {
                    println!(
                        "{}{}",
                        index
                            .map(|index| index.to_string() + ":")
                            .unwrap_or(String::from("")),
                        matched_line
                    );
                };
            };
            (index, matched_count)
        },
    );

    match has_matched {
        Ok(match_count) => {
            if config.print_only_count {
                println!("{}", match_count.expect("failed to count matched input"));
            }
            std::process::exit(0);
        }
        Err(_) => {
            std::process::exit(1);
        }
    }
}
