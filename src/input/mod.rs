use colored::*;
use json::JsonValue;
use json_highlight_writer::{highlight, highlight_with_colors};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind};
use std::result::Result;
use std::string::String;

mod enumeration;
use crate::selection::match_json_slice;
use crate::selection::SelectionLens;

pub enum HighlightMatches {
    Never,
    Cycle,
    Single,
}

pub struct Config<'a> {
    pub matchers: Vec<&'a str>,
    pub input: Option<&'a str>,
    pub print_only_count: bool,
    pub print_line_number: bool,
    pub highlight_matches: HighlightMatches,
    pub ignore_case: bool,
    pub is_quiet_mode: bool,
    pub invert_match: bool,
    pub match_root_only: bool,
    pub max_num: Option<usize>,
}

pub fn match_input<'a>(
    config: &Config,
    on_line: &Fn(String) -> Result<String, String>,
    on_result: &Fn(
        (Option<usize>, Option<usize>, Result<String, String>),
    ) -> (Option<usize>, Option<usize>),
) -> Result<Option<usize>, Option<String>> {
    let stdin = io::stdin();
    let input = match config.input {
        Some(input) => buffer_input_file(input)?,
        None => Box::new(stdin.lock()) as Box<BufRead>,
    };

    let mut enumerate = enumeration::enumeration(
        config.print_line_number,
        config.print_only_count || config.max_num.is_some(),
    );

    input
        .lines()
        .map(|line: Result<String, Error>| {
            on_line(line.expect("Could not read line from standard in"))
        })
        .map(|res| enumerate(res))
        .filter(|(_, _, match_result)| match_result.is_ok())
        .map(|res| on_result(res))
        .take_while(|(_, matched_lines)| match config.max_num {
            Some(max) => matched_lines.map(|matched| matched < max).unwrap_or(true),
            None => true,
        })
        .last()
        .map(|(_, matched_lines)| Ok(matched_lines))
        .unwrap_or(Err(None))
}

fn buffer_input_file(input: &str) -> Result<Box<BufRead>, Option<String>> {
    match File::open(input) {
        Ok(contents) => Ok(Box::new(BufReader::new(contents))),
        Err(error) => Err(match error.kind() {
            ErrorKind::NotFound => Some(format!(
                "The specified input file could not be found: {:?}",
                input
            )),
            other_error => Some(format!(
                "There was a problem opening the file '{:?}': {:?}",
                input, other_error
            )),
        }),
    }
}

pub fn in_configured_case(value: &str, config: &Config) -> String {
    if config.ignore_case {
        value.to_lowercase()
    } else {
        String::from(value)
    }
}

pub fn match_line(
    matchers: &Vec<Vec<Box<SelectionLens>>>,
    config: &Config,
    input: String,
) -> Result<String, String> {
    match json::parse(&in_configured_case(&input, config)) {
        Ok(json_input) => {
            let matches: Vec<&JsonValue> = matchers
                .iter()
                .map(|selector| match_json_slice(selector, &json_input, config.match_root_only))
                .filter_map(Result::ok)
                .collect();

            if matches.is_empty() {
                Err(input)
            } else {
                Ok(match config.highlight_matches {
                    HighlightMatches::Never => input,
                    HighlightMatches::Single => highlight(&json_input, matches),
                    HighlightMatches::Cycle => highlight_with_colors(
                        &json_input,
                        matches,
                        vec![
                            Color::Red,
                            Color::Blue,
                            Color::Yellow,
                            Color::Green,
                            Color::Magenta,
                            Color::Cyan,
                        ],
                    ),
                })
            }
        }
        _ => Err(input),
    }
}
