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
                .help("JSON selector pattern")
        )
        .arg(
            Arg::with_name("patterns")
                .multiple(true)
                .takes_value(true)
                .short("e")
                .long("pattern")
                .help("JSON selector pattern")
        )
        .arg(
            Arg::with_name("match-root")
                .short("^")
                .long("match-root")
                .help("Select lines whose JSON input matches from the root of the object.")
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
            Arg::with_name("invert-match")
                .short("v")
                .long("invert-match")
                .help("Selected lines are those _not_ matching any of the specified selector patterns.")
        )
        .get_matches();

    let config = input::Config {
        print_only_count: matches.is_present("count"),
        print_line_number: matches.is_present("line-number"),
        ignore_case: matches.is_present("ignore-case"),
        is_quiet_mode: matches.is_present("quiet"),
        match_root_only: matches.is_present("match-root"),
        invert_match: matches.is_present("invert-match"),
        max_num: matches.value_of("max-count").map(|num| {
            usize::from_str_radix(num, 32).expect("an invalid -m/--max-num flag has been specified")
        }),
    };

    let matched_filters = matches
        .values_of("patterns")
        .map(|values| values.collect::<Vec<_>>())
        .or_else(|| {
            matches
                .value_of("pattern")
                .or(Some("."))
                .map(|pattern| vec![pattern])
        })
        .map(|patterns| {
            patterns
                .iter()
                .map(|pattern| {
                    input::in_configured_case(pattern, &config).unwrap_or(String::from(*pattern))
                })
                .map(|pattern| selection::match_filters(&pattern))
                .collect::<Vec<_>>()
        })
        .expect("No matcher pattern has been specified");

    let has_matched = input::match_input(
        matches.value_of("file"),
        &config,
        &|line| {
            invert_result(
                config.invert_match,
                input::match_line(&matched_filters, &config, line),
            )
        },
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

fn invert_result<A>(should_invert: bool, result: Result<A, A>) -> Result<A, A> {
    match result {
        Ok(ok_res) => {
            if should_invert {
                Err(ok_res)
            } else {
                Ok(ok_res)
            }
        }
        Err(err_res) => {
            if should_invert {
                Ok(err_res)
            } else {
                Err(err_res)
            }
        }
    }
}
