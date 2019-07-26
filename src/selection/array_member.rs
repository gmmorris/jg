use json::JsonValue;
use regex::Regex;

use super::{value_matchers::*, SelectionLens, SelectionLensParser};

struct ArrayIndexMember {
    index: usize,
}

impl SelectionLens for ArrayIndexMember {
    fn select<'a>(&self, input: Option<&'a JsonValue>) -> Option<&'a JsonValue> {
        match input {
            Some(json) => match json {
                JsonValue::Array(ref array) => array.get(self.index),
                _ => None,
            },
            None => None,
        }
    }
}

struct ArrayValueMember {
    value: JsonValueMemberMatcher,
}

impl ArrayValueMember {
    pub fn member_in_array<'a>(
        sequence: &'a Vec<JsonValue>,
        json_value_matcher: &JsonValueMatcher,
    ) -> Option<&'a JsonValue> {
        sequence
            .iter()
            .find(|member| match (member, json_value_matcher) {
                (JsonValue::Short(string_prop), JsonValueMatcher::String(string_value)) => {
                    string_value.eq(string_prop)
                }
                (JsonValue::String(string_prop), JsonValueMatcher::String(string_value)) => {
                    string_value.eq(string_prop)
                }
                (JsonValue::Boolean(bool_prop), JsonValueMatcher::Boolean(bool_value)) => {
                    bool_prop.eq(bool_value)
                }
                (JsonValue::Number(num_prop), JsonValueMatcher::Number(num_value)) => {
                    num_prop.eq(num_value)
                }
                (JsonValue::Null, JsonValueMatcher::Null) => true,
                _ => false,
            })
    }
}

impl SelectionLens for ArrayValueMember {
    fn select<'a>(&self, input: Option<&'a JsonValue>) -> Option<&'a JsonValue> {
        match input {
            Some(JsonValue::Array(ref array)) => match &self.value {
                JsonValueMemberMatcher::Exact(json_value_matcher) => {
                    if array.len() == 1 {
                        ArrayValueMember::member_in_array(array, json_value_matcher)
                    } else {
                        None
                    }
                }
                JsonValueMemberMatcher::ContainsExact(json_value_matcher) => {
                    ArrayValueMember::member_in_array(array, json_value_matcher)
                }
                JsonValueMemberMatcher::Prefixed(json_value_matcher) => {
                    array
                        .iter()
                        .find(|member| match (member, json_value_matcher) {
                            (
                                JsonValue::Short(string_prop),
                                JsonValueMatcher::String(string_value),
                            ) => string_prop.starts_with(string_value),
                            (
                                JsonValue::String(string_prop),
                                JsonValueMatcher::String(string_value),
                            ) => string_prop.starts_with(string_value),
                            _ => false,
                        })
                }
                JsonValueMemberMatcher::Suffixed(json_value_matcher) => {
                    array
                        .iter()
                        .find(|member| match (member, json_value_matcher) {
                            (
                                JsonValue::Short(string_prop),
                                JsonValueMatcher::String(string_value),
                            ) => string_prop.ends_with(string_value),
                            (
                                JsonValue::String(string_prop),
                                JsonValueMatcher::String(string_value),
                            ) => string_prop.ends_with(string_value),
                            _ => false,
                        })
                }
                JsonValueMemberMatcher::Contains(json_value_matcher) => {
                    array
                        .iter()
                        .find(|member| match (member, json_value_matcher) {
                            (
                                JsonValue::Short(string_prop),
                                JsonValueMatcher::String(string_value),
                            ) => string_prop.contains(string_value),
                            (
                                JsonValue::String(string_prop),
                                JsonValueMatcher::String(string_value),
                            ) => string_prop.contains(string_value),
                            _ => false,
                        })
                }
            },
            _ => None,
        }
    }
}

enum ArrayMember {
    Index(usize),
    Value(JsonValueMemberMatcher),
}

pub struct ArrayMemberParser;
impl ArrayMemberParser {
    fn match_array_member(pattern: &str) -> Option<(ArrayMember, Option<&str>)> {
        lazy_static! {
        static ref RE_INDEX: Regex =
            Regex::new(r#"^\[(?P<index>([[:digit:]])+)\](?P<remainder>.+)?$"#).unwrap();
        static ref RE_MEMBER: Regex = Regex::new(
            concat!(r#"^\["#,r#"(?P<matchingStrategy>(~=|=|\$=|\^=|\*=)+)"#,r#"("(?P<stringValue>([^"])+)"|(?P<numberValue>([[:digit:]]+)+)|(?P<literalValue>([[:word:]])+))\](?P<remainder>.+)?$"#)
        )
        .unwrap();
        }

        match RE_INDEX.captures(pattern) {
            Some(captured_index) => captured_index
                .name("index")
                .map(|index| index.as_str())
                .map(|index| usize::from_str_radix(index, 32))
                .filter(|index| index.is_ok())
                .map(|index| {
                    (
                        ArrayMember::Index(index.unwrap()),
                        captured_index
                            .name("remainder")
                            .map(|remainder| remainder.as_str()),
                    )
                }),
            None => match RE_MEMBER.captures(pattern) {
                Some(cap) => match identify_value_matcher(&cap) {
                    Ok(Some(json_matcher)) => Some((
                        ArrayMember::Value(json_matcher),
                        cap.name("remainder").map(|remainder| remainder.as_str()),
                    )),
                    Ok(None) => None,
                    Err(_) => None,
                },
                None => None,
            },
        }
    }
}
impl SelectionLensParser for ArrayMemberParser {
    fn try_parse<'a>(
        &self,
        lens_pattern: Option<&'a str>,
    ) -> Result<(Box<SelectionLens>, Option<&'a str>), Option<&'a str>> {
        match lens_pattern {
            Some(pattern) => match ArrayMemberParser::match_array_member(pattern) {
                Some((array_member, remainder)) => Ok((
                    match array_member {
                        ArrayMember::Index(index) => Box::new(ArrayIndexMember { index }),
                        ArrayMember::Value(value) => Box::new(ArrayValueMember { value }),
                    },
                    remainder,
                )),
                None => Err(lens_pattern),
            },
            None => Err(lens_pattern),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use json::array;
    use json::object;

    #[test]
    fn should_match_array_index() {
        let array_member_parser = ArrayMemberParser {};
        let res = array_member_parser.try_parse(Some("[0]"));
        assert!(res.is_ok());

        let ref data = array![object! {
            "name"    => "John Doe",
            "age"     => 30
        }];

        match res {
            Ok((matcher, _)) => assert_eq!(matcher.select(Some(data)), Some(&data[0])),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_return_remainder_when_it_matches_index() {
        let array_member_parser = ArrayMemberParser {};
        let res = array_member_parser.try_parse(Some("[0].title"));
        assert!(res.is_ok());

        match res {
            Ok((_, umatched)) => assert_eq!(umatched, Some(".title")),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_return_node_when_json_is_present() {
        let ref data = array![object! {
            "name"    => "John Doe",
            "age"     => 30
        }];

        let array_index = ArrayIndexMember { index: 0 };
        assert_eq!(array_index.select(Some(data)), Some(&data[0]));
    }

    #[test]
    fn should_return_none_when_json_isnt_present() {
        let array_index = ArrayIndexMember { index: 10 };
        assert_eq!(array_index.select(None), None);
    }

    #[test]
    fn should_return_node_when_exact_string_value_is_only_value_in_array() {
        let ref data = array!["Jane Doe"];

        let array_member = ArrayValueMember {
            value: JsonValueMemberMatcher::Exact(JsonValueMatcher::String(String::from(
                "Jane Doe",
            ))),
        };
        assert_eq!(array_member.select(Some(data)), Some(&data[0]));
    }

    #[test]
    fn should_return_node_when_exact_string_value_is_contained_in_array() {
        let ref data = array!["John Doe", "Jane Doe", "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."];

        let array_member = ArrayValueMember {
            value: JsonValueMemberMatcher::ContainsExact(JsonValueMatcher::String(String::from(
                "Jane Doe",
            ))),
        };
        assert_eq!(array_member.select(Some(data)), Some(&data[1]));

        let array_member = ArrayValueMember {
            value: JsonValueMemberMatcher::ContainsExact(JsonValueMatcher::String(String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")))
        };
        assert_eq!(array_member.select(Some(data)), Some(&data[2]));
    }

    #[test]
    fn should_return_node_when_boolean_value_is_present_in_array() {
        let ref data = array![true, false];

        let array_member = ArrayValueMember {
            value: JsonValueMemberMatcher::ContainsExact(JsonValueMatcher::Boolean(true)),
        };
        assert_eq!(array_member.select(Some(data)), Some(&data[0]));

        let array_member = ArrayValueMember {
            value: JsonValueMemberMatcher::ContainsExact(JsonValueMatcher::Boolean(false)),
        };
        assert_eq!(array_member.select(Some(data)), Some(&data[1]));
    }

    #[test]
    fn should_return_node_when_null_value_is_present_in_array() {
        let ref data = array![true, JsonValue::Null];

        let array_member = ArrayValueMember {
            value: JsonValueMemberMatcher::ContainsExact(JsonValueMatcher::Null),
        };
        assert_eq!(array_member.select(Some(data)), Some(&data[1]));
    }

    #[test]
    fn should_return_node_when_numeric_value_is_present_in_array() {
        let ref data = array![0, -10, 10, 123456789];

        let array_member = ArrayValueMember {
            value: JsonValueMemberMatcher::ContainsExact(JsonValueMatcher::Number(0)),
        };
        assert_eq!(array_member.select(Some(data)), Some(&data[0]));

        let array_member = ArrayValueMember {
            value: JsonValueMemberMatcher::ContainsExact(JsonValueMatcher::Number(-10)),
        };
        assert_eq!(array_member.select(Some(data)), Some(&data[1]));

        let array_member = ArrayValueMember {
            value: JsonValueMemberMatcher::ContainsExact(JsonValueMatcher::Number(10)),
        };
        assert_eq!(array_member.select(Some(data)), Some(&data[2]));

        let array_member = ArrayValueMember {
            value: JsonValueMemberMatcher::ContainsExact(JsonValueMatcher::Number(123456789)),
        };
        assert_eq!(array_member.select(Some(data)), Some(&data[3]));
    }
}
