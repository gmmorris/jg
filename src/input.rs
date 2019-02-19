use json::JsonValue;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, BufRead, BufReader, Error};
use std::result::Result;
use std::string::String;

pub fn match_input(
  input_file: Option<&str>,
  filters: Vec<Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>>>,
  match_line: &Fn(&Vec<Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>>>, Result<String, Error>),
) {
  let stdin = io::stdin();
  let input = match input_file {
    Some(input) => buffer_input_file(input),
    None => Box::new(stdin.lock()) as Box<BufRead>,
  };

  for line in input.lines() {
    match_line(&filters, line)
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
