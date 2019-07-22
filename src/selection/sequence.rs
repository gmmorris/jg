use json::JsonValue;
use regex::Regex;

use super::{match_json_slice, try_to_match_filters, FnJsonValueLens};

pub fn sequence(matchers: Vec<Box<FnJsonValueLens>>) -> Box<FnJsonValueLens> {
    Box::new(move |input: Option<&JsonValue>| match input {
        Some(json) => match json {
            JsonValue::Array(ref array) => array
                .iter()
                .find(|member| match_json_slice(&matchers, member, true).is_ok()),
            _ => None,
        },
        None => None,
    })
}

fn match_sequence(pattern: &str) -> Option<&str> {
    lazy_static! {
        static ref RE_SEQUENCE: Regex = Regex::new(r#"^\[(?P<sequence_matcher>(.)+)\]$"#).unwrap();
    }

    RE_SEQUENCE
        .captures(pattern)
        .and_then(|cap| cap.name("sequence_matcher"))
        .map(|sequence_matcher| sequence_matcher.as_str())
}

pub fn greedily_matches(
    maybe_pattern: Option<&str>,
) -> Result<(Box<FnJsonValueLens>, Option<&str>), Option<&str>> {
    match maybe_pattern
        .and_then(match_sequence)
        .map(try_to_match_filters)
    {
        Some(Ok(matchers)) => Ok((sequence(matchers), None)),
        _ => Err(maybe_pattern),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use json::array;
    use json::object;

    #[test]
    fn should_match_json_in_sequence_when_matching_query() {
        let res = greedily_matches(Some("[.name]"));
        assert!(res.is_ok());

        let ref data = object! {
          "name"    => "John Doe",
          "age"     => 30,
          "identities" => array![object! {
              "name"    => "Richard Roe"
          }]
        };

        match res {
            Ok((matcher, _)) => assert_eq!(
                matcher(Some(&data["identities"])),
                Some(&data["identities"][0])
            ),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_return_none_when_json_sequence_is_empty() {
        let ref data = object! {
          "name"    => "John Doe",
          "age"     => 30,
          "identities" => array![]
        };
        assert_eq!(
            sequence(try_to_match_filters(".").unwrap())(Some(&data["identities"])),
            None
        );
    }
}
