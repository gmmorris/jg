use json::JsonValue;
use regex::Regex;

use super::{match_json_slice, try_to_match_filters, SelectionLens, SelectionLensParser};

struct Sequence {
    matchers: Vec<Box<dyn SelectionLens>>,
}

impl SelectionLens for Sequence {
    fn select<'a>(&self, input: Option<&'a JsonValue>) -> Option<&'a JsonValue> {
        match input {
            Some(json) => match json {
                JsonValue::Array(ref array) => array
                    .iter()
                    .find(|member| match_json_slice(&self.matchers, member, true).is_ok()),
                _ => None,
            },
            None => None,
        }
    }
}

pub struct SequenceParser;
impl SequenceParser {
    fn match_sequence(pattern: &str) -> Option<&str> {
        lazy_static! {
            static ref RE_SEQUENCE: Regex =
                Regex::new(r#"^\[(?P<sequence_matcher>(.)+)\]$"#).unwrap();
        }

        RE_SEQUENCE
            .captures(pattern)
            .and_then(|cap| cap.name("sequence_matcher"))
            .map(|sequence_matcher| sequence_matcher.as_str())
    }
}
impl SelectionLensParser for SequenceParser {
    fn try_parse<'a>(
        &self,
        lens_pattern: Option<&'a str>,
    ) -> Result<(Box<dyn SelectionLens>, Option<&'a str>), Option<&'a str>> {
        match lens_pattern
            .and_then(SequenceParser::match_sequence)
            .map(try_to_match_filters)
        {
            Some(Ok(matchers)) => Ok((Box::new(Sequence { matchers }), None)),
            _ => Err(lens_pattern),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use json::array;
    use json::object;

    #[test]
    fn should_match_json_in_sequence_when_matching_query() {
        let sequence_parser = SequenceParser {};
        let res = sequence_parser.try_parse(Some("[.name]"));
        assert!(res.is_ok());

        let data = &object! {
          "name"    => "John Doe",
          "age"     => 30,
          "identities" => array![object! {
              "name"    => "Richard Roe"
          }]
        };

        match res {
            Ok((matcher, _)) => assert_eq!(
                matcher.select(Some(&data["identities"])),
                Some(&data["identities"][0])
            ),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_return_none_when_json_sequence_is_empty() {
        let data = &object! {
          "name"    => "John Doe",
          "age"     => 30,
          "identities" => array![]
        };

        let sequence = Sequence {
            matchers: try_to_match_filters(".").unwrap(),
        };

        assert_eq!(sequence.select(Some(&data["identities"])), None);
    }
}
