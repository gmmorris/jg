use json::JsonValue;
use regex::Regex;

use super::value_matchers::*;
use super::{SelectionLens, SelectionLensParser};

struct Prop {
    name: String,
    value: Option<JsonValueMemberMatcher>,
}

impl Prop {
    pub fn prop_value_matches_exact<'a, 'b>(
        prop: &'a JsonValue,
        prop_value_matcher: &'b JsonValueMatcher,
    ) -> Option<&'a JsonValue> {
        match (prop, prop_value_matcher) {
            (&JsonValue::String(ref string_prop), &JsonValueMatcher::String(ref prop_value)) => {
                Some(prop).filter(|_| string_prop.eq(prop_value))
            }
            (&JsonValue::Short(ref string_prop), &JsonValueMatcher::String(ref prop_value)) => {
                Some(prop).filter(|_| string_prop.eq(prop_value))
            }
            (&JsonValue::Number(ref number_prop), &JsonValueMatcher::Number(ref prop_value)) => {
                Some(prop).filter(|_| number_prop.eq(prop_value))
            }
            (&JsonValue::Boolean(ref bool_prop), &JsonValueMatcher::Boolean(ref prop_value)) => {
                Some(prop).filter(|_| bool_prop.eq(prop_value))
            }
            (&JsonValue::Null, &JsonValueMatcher::Null) => Some(prop),
            (_, _) => None,
        }
    }

    pub fn prop_value_contains_exact<'a, 'b>(
        prop: &'a JsonValue,
        prop_value_matcher: &'b JsonValueMatcher,
    ) -> Option<&'a JsonValue> {
        match (prop, prop_value_matcher) {
            (&JsonValue::String(ref string_prop), &JsonValueMatcher::String(ref prop_value)) => {
                Some(prop).filter(|_| {
                    string_prop
                        .split_whitespace()
                        .any(|string_prop| string_prop.eq(prop_value))
                })
            }
            (&JsonValue::Short(ref string_prop), &JsonValueMatcher::String(ref prop_value)) => {
                Some(prop).filter(|_| {
                    string_prop
                        .split_whitespace()
                        .any(|string_prop| string_prop.eq(prop_value))
                })
            }
            (_, _) => None,
        }
    }

    pub fn prop_value_is_prefixed_by<'a, 'b>(
        prop: &'a JsonValue,
        prop_value_matcher: &'b JsonValueMatcher,
    ) -> Option<&'a JsonValue> {
        match (prop, prop_value_matcher) {
            (&JsonValue::String(ref string_prop), &JsonValueMatcher::String(ref prop_value)) => {
                Some(prop).filter(|_| string_prop.starts_with(prop_value))
            }
            (&JsonValue::Short(ref string_prop), &JsonValueMatcher::String(ref prop_value)) => {
                Some(prop).filter(|_| string_prop.starts_with(prop_value))
            }
            (_, _) => None,
        }
    }

    pub fn prop_value_is_suffixed_by<'a, 'b>(
        prop: &'a JsonValue,
        prop_value_matcher: &'b JsonValueMatcher,
    ) -> Option<&'a JsonValue> {
        match (prop, prop_value_matcher) {
            (&JsonValue::String(ref string_prop), &JsonValueMatcher::String(ref prop_value)) => {
                Some(prop).filter(|_| string_prop.ends_with(prop_value))
            }
            (&JsonValue::Short(ref string_prop), &JsonValueMatcher::String(ref prop_value)) => {
                Some(prop).filter(|_| string_prop.ends_with(prop_value))
            }
            (_, _) => None,
        }
    }

    pub fn prop_value_contains<'a, 'b>(
        prop: &'a JsonValue,
        prop_value_matcher: &'b JsonValueMatcher,
    ) -> Option<&'a JsonValue> {
        match (prop, prop_value_matcher) {
            (&JsonValue::String(ref string_prop), &JsonValueMatcher::String(ref prop_value)) => {
                Some(prop).filter(|_| string_prop.contains(prop_value))
            }
            (&JsonValue::Short(ref string_prop), &JsonValueMatcher::String(ref prop_value)) => {
                Some(prop).filter(|_| string_prop.contains(prop_value))
            }
            (_, _) => None,
        }
    }
}

impl SelectionLens for Prop {
    fn select<'a>(&self, input: Option<&'a JsonValue>) -> Option<&'a JsonValue> {
        match input {
            Some(JsonValue::Object(ref object)) => match (object.get(&self.name), &self.value) {
                (Some(prop), Some(JsonValueMemberMatcher::Exact(prop_value_matcher))) => {
                    Prop::prop_value_matches_exact(prop, prop_value_matcher)
                }
                (Some(prop), Some(JsonValueMemberMatcher::ContainsExact(prop_value_matcher))) => {
                    Prop::prop_value_contains_exact(prop, prop_value_matcher)
                }
                (Some(prop), Some(JsonValueMemberMatcher::Prefixed(prop_value_matcher))) => {
                    Prop::prop_value_is_prefixed_by(prop, prop_value_matcher)
                }
                (Some(prop), Some(JsonValueMemberMatcher::Suffixed(prop_value_matcher))) => {
                    Prop::prop_value_is_suffixed_by(prop, prop_value_matcher)
                }
                (Some(prop), Some(JsonValueMemberMatcher::Contains(prop_value_matcher))) => {
                    Prop::prop_value_contains(prop, prop_value_matcher)
                }
                (Some(prop), None) => Some(prop),
                (None, _) => None,
            },
            _ => None,
        }
    }
}

pub struct PropParser;
impl PropParser {
    fn match_prop(pattern: &str) -> Option<(&str, Option<JsonValueMemberMatcher>, Option<&str>)> {
        lazy_static! {
            static ref RE_PROP: Regex =
                Regex::new(r#"^\.(?P<prop>([[:word:]])+)(?P<remainder>.+)?$"#).unwrap();
            static ref RE_PROP_VALUE: Regex = Regex::new(
                concat!(r#"^\{"(?P<prop>([[:word:]])+)"("#,r#"(?P<matchingStrategy>(:|~:|\$:|\^:|\*:)+)"#,r#"("(?P<stringValue>([^"])+)"|(?P<numberValue>([[:digit:]]+)+)|(?P<literalValue>([[:word:]])+)))?\}(?P<remainder>.+)?$"#)
            )
            .unwrap();
        }

        match RE_PROP
            .captures(pattern)
            .or(RE_PROP_VALUE.captures(pattern))
        {
            Some(cap) => cap
                .name("prop")
                .and_then(|prop| match identify_value_matcher(&cap) {
                    Ok(json_matcher) => Some((
                        prop.as_str(),
                        json_matcher,
                        cap.name("remainder").map(|remainder| remainder.as_str()),
                    )),
                    Err(_) => None,
                }),
            None => None,
        }
    }
}
impl SelectionLensParser for PropParser {
    fn try_parse<'a>(
        &self,
        lens_pattern: Option<&'a str>,
    ) -> Result<(Box<dyn SelectionLens>, Option<&'a str>), Option<&'a str>> {
        match lens_pattern {
            Some(pattern) => match PropParser::match_prop(pattern) {
                Some((prop_name, prop_value, remainder)) => Ok((
                    Box::new(Prop {
                        name: String::from(prop_name),
                        value: prop_value,
                    }),
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
    use json::object;

    #[test]
    fn should_match_prop() {
        let prop_parser = PropParser {};
        let res = prop_parser.try_parse(Some(".name"));
        assert!(res.is_ok());

        let data = &object! {
            "name"    => "John Doe",
            "age"     => 30
        };

        match res {
            Ok((matcher, _)) => assert_eq!(matcher.select(Some(data)), Some(&data["name"])),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn shouldnt_match_identity() {
        let prop_parser = PropParser {};
        let res = prop_parser.try_parse(Some("."));
        assert!(res.is_err());
        match res {
            Err(Some(selector)) => assert_eq!(selector, "."),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_return_remainder_when_it_matches_prop() {
        let prop_parser = PropParser {};
        let res = prop_parser.try_parse(Some(".father.title"));
        assert!(res.is_ok());

        match res {
            Ok((_, umatched)) => assert_eq!(umatched, Some(".title")),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_return_none_when_json_isnt_present() {
        let prop = Prop {
            name: String::from(".id"),
            value: None,
        };
        assert_eq!(prop.select(None), None);
    }

    #[test]
    fn should_return_json_prop_when_json_has_prop() {
        let data = &object! {
            "name"    => "John Doe",
            "age"     => 30
        };

        let prop = Prop {
            name: String::from("name"),
            value: None,
        };

        assert_eq!(prop.select(Some(data)), Some(&data["name"]));
    }

    #[test]
    fn should_match_number_prop() {
        let prop_parser = PropParser {};
        let res = prop_parser.try_parse(Some(r#"{"age":30}"#));
        assert!(res.is_ok());

        let data = &object! {
            "name"    => "John Doe",
            "age"     => 30
        };

        match res {
            Ok((matcher, _)) => assert_eq!(matcher.select(Some(data)), Some(&data["age"])),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_match_string_property_value_when_using_exact_matching_strategy() {
        let prop_parser = PropParser {};
        let res = prop_parser.try_parse(Some(r#"{"country":"IRL"}"#));
        assert!(res.is_ok());

        let data = &object! {
            "name"      => "John Doe",
            "age"       => 30,
            "country"   => "IRL"
        };

        match res {
            Ok((matcher, _)) => assert_eq!(matcher.select(Some(data)), Some(&data["country"])),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_match_string_property_value_when_using_contains_exact_matching_strategy() {
        let prop_parser = PropParser {};
        let res = prop_parser.try_parse(Some(r#"{"country"~:"GBR"}"#));
        assert!(res.is_ok());

        let data = &object! {
            "name"      => "John Doe",
            "age"       => 30,
            "country"   => "IRL GBR"
        };

        match res {
            Ok((matcher, _)) => assert_eq!(matcher.select(Some(data)), Some(&data["country"])),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_match_string_property_value_when_using_prefixed_matching_strategy() {
        let prop_parser = PropParser {};
        let res = prop_parser.try_parse(Some(r#"{"country"^:"IRL"}"#));
        assert!(res.is_ok());

        let data = &object! {
            "name"      => "John Doe",
            "age"       => 30,
            "country"   => "IRL GBR"
        };

        match res {
            Ok((matcher, _)) => assert_eq!(matcher.select(Some(data)), Some(&data["country"])),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_match_string_property_value_when_using_suffixed_matching_strategy() {
        let prop_parser = PropParser {};
        let res = prop_parser.try_parse(Some(r#"{"country"$:"GBR"}"#));
        assert!(res.is_ok());

        let data = &object! {
            "name"      => "John Doe",
            "age"       => 30,
            "country"   => "IRL GBR"
        };

        match res {
            Ok((matcher, _)) => assert_eq!(matcher.select(Some(data)), Some(&data["country"])),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_match_boolean_false_prop() {
        let prop_parser = PropParser {};
        let res = prop_parser.try_parse(Some(r#"{"is_known":false}"#));
        assert!(res.is_ok());

        let data = &object! {
            "name"      => "John Doe",
            "age"       => 30,
            "is_known"  => false
        };

        match res {
            Ok((matcher, _)) => assert_eq!(matcher.select(Some(data)), Some(&data["is_known"])),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_match_boolean_true_prop() {
        let prop_parser = PropParser {};
        let res = prop_parser.try_parse(Some(r#"{"is_anonymous":true}"#));
        assert!(res.is_ok());

        let data = &object! {
            "name"          => "John Doe",
            "age"           => 30,
            "is_anonymous"  => true
        };

        match res {
            Ok((matcher, _)) => assert_eq!(matcher.select(Some(data)), Some(&data["is_anonymous"])),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_match_null_prop() {
        let prop_parser = PropParser {};
        let res = prop_parser.try_parse(Some(r#"{"identity":null}"#));
        assert!(res.is_ok());

        let data = &object! {
            "name"          => "John Doe",
            "age"           => 30,
            "identity"      => JsonValue::Null
        };

        match res {
            Ok((matcher, _)) => assert_eq!(matcher.select(Some(data)), Some(&data["identity"])),
            _ => panic!("Invalid result"),
        }
    }
}
