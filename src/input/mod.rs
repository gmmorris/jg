use json::JsonValue;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind};
use std::result::Result;
use std::string::String;

use crate::selection::match_json_slice;

pub struct Config {
  pub print_only_count: bool,
  pub ignore_case: bool,
  pub max_num: Option<usize>,
}

pub fn match_input(
  input_file: Option<&str>,
  config: &Config,
  on_line: &Fn(String) -> Result<(), ()>,
) -> Result<Option<u64>, ()> {
  let stdin = io::stdin();
  let input = match input_file {
    Some(input) => buffer_input_file(input),
    None => Box::new(stdin.lock()) as Box<BufRead>,
  };

  let process =
    |line: Result<String, Error>| on_line(line.expect("Could not read line from standard in"));

  let reduce_to_count =
    |count: Result<Option<u64>, ()>, match_result: Result<(), ()>| match match_result {
      Ok(()) => match count {
        Ok(Some(count)) => Ok(Some(
          count
            .checked_add(1)
            .expect("failed to count matches, likely overflowed"),
        )),
        Ok(None) => Ok(None),
        Err(()) => Ok(if config.print_only_count {
          Some(1)
        } else {
          None
        }),
      },
      Err(()) => count,
    };

  if let Some(lim) = config.max_num {
    input
      .lines()
      .map(process)
      .filter(|match_result| match_result.is_ok())
      .take(lim)
      .fold(Err(()), reduce_to_count)
  } else {
    input
      .lines()
      .map(process)
      .filter(|match_result| match_result.is_ok())
      .fold(Err(()), reduce_to_count)
  }
}

fn buffer_input_file(input: &str) -> Box<BufRead> {
  match File::open(input) {
    Ok(contents) => Box::new(BufReader::new(contents)),
    Err(error) => match error.kind() {
      ErrorKind::NotFound => panic!("The specified input file could not be found: {:?}", input),
      other_error => panic!(
        "There was a problem opening the file '{:?}': {:?}",
        input, other_error
      ),
    },
  }
}

pub fn in_configured_case(value: &str, config: &Config) -> Option<String> {
  if config.ignore_case {
    Some(value.to_lowercase())
  } else {
    None
  }
}

pub fn match_line(
  matchers: &Vec<Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>>>,
  config: &Config,
  input: String,
) -> Result<String, ()> {
  let parsed_json = in_configured_case(&input, config)
    .map(|configured_string| json::parse(&configured_string))
    .unwrap_or(json::parse(&input));
  match parsed_json {
    Ok(json_input) => match match_json_slice(matchers, &json_input) {
      Ok(_) => Ok(input),
      _ => Err(()),
    },
    _ => Err(()),
  }
}
