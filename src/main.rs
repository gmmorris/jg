extern crate jg;
use jg::input::HighlightMatches;

use clap::{crate_version, App, Arg};
use isatty::stdout_isatty;

fn main() {
    let matches = App::new("jg")
        .version(crate_version!())
        .author("Gidi Meir Morris <gidi@gidi.io>")
        .about("Jeff Goldblum (jg) searches for PATTERNS in json input, jgrep prints each json object that matches a pattern.")
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
            Arg::with_name("colour")
                .long("colour")
                .visible_alias("color")
                .takes_value(true)
                .possible_values(&["never", "auto", "auto-cycle", "always", "always-cycle"])
                .help("Mark up the JSON shapes matching the selector pattern when printing the output.")
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
            Arg::with_name("params")
                .multiple(true)
                .takes_value(true)
                .short("p")
                .long("params")
                .help("Parameters to be substituted within the specified pattern")
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .visible_alias("silent")
                .help("Quiet mode: suppress normal output.")
        )
        .arg(
            Arg::with_name("invert-match")
                .short("v")
                .long("invert-match")
                .help("Selected lines are those _not_ matching any of the specified selector patterns.")
        )
        .get_matches();

    let matched_filters = matches
        .values_of("patterns")
        .map(|values| values.collect::<Vec<_>>())
        .or_else(|| {
            matches
                .value_of("pattern")
                .or(Some("."))
                .map(|pattern| vec![pattern])
        })
        .expect("No matcher pattern has been specified");


    let config = jg::input::Config {
        matchers: matched_filters,
        params: matches.values_of("params").map(|values| values.collect::<Vec<_>>()),
        input: matches.value_of("file"),
        print_only_count: matches.is_present("count"),
        highlight_matches: match (matches.value_of("colour"), stdout_isatty()) {
            (Some("always"), _) | (Some("auto"), true) => HighlightMatches::Single,
            (Some("always-cycle"), _) | (Some("auto-cycle"), true) => HighlightMatches::Cycle,
            _ => HighlightMatches::Never,
        },
        print_line_number: matches.is_present("line-number"),
        ignore_case: matches.is_present("ignore-case"),
        is_quiet_mode: matches.is_present("quiet"),
        match_root_only: matches.is_present("match-root"),
        invert_match: matches.is_present("invert-match"),
        max_num: matches.value_of("max-count").map(|num| {
            usize::from_str_radix(num, 32).expect("an invalid -m/--max-num flag has been specified")
        }),
    };

    std::process::exit(match jg::json_grep(config) {
        Ok(_) => 0,
        Err(Some(err)) => {
            eprintln!("{:}", err);
            1
        }
        Err(None) => {
            eprintln!("Unknown error");
            1
        }
    });
}
