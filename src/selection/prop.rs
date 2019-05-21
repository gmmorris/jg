use json::JsonValue;
use regex::Regex;

use super::value_matchers::*;

fn prop_value_matches_exact<'a, 'b>(
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

fn prop_value_contains_exact<'a, 'b>(
    prop: &'a JsonValue,
    prop_value_matcher: &'b JsonValueMatcher,
) -> Option<&'a JsonValue> {
    match (prop, prop_value_matcher) {
        (&JsonValue::String(ref string_prop), &JsonValueMatcher::String(ref prop_value)) => {
            Some(prop).filter(|_| {
                string_prop
                    .split_whitespace()
                    .find(|string_prop| string_prop.eq(prop_value))
                    .is_some()
            })
        }
        (&JsonValue::Short(ref string_prop), &JsonValueMatcher::String(ref prop_value)) => {
            Some(prop).filter(|_| {
                string_prop
                    .split_whitespace()
                    .find(|string_prop| string_prop.eq(prop_value))
                    .is_some()
            })
        }
        (_, _) => None,
    }
}

fn prop_value_is_prefixed_by<'a, 'b>(
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

fn prop_value_is_suffixed_by<'a, 'b>(
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

fn prop_value_contains<'a, 'b>(
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

pub fn prop(
    prop_name: String,
    prop_value: Option<JsonValueMemberMatcher>,
) -> Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>> {
    Box::new(move |input: Option<&JsonValue>| match input {
        Some(JsonValue::Object(ref object)) => match (object.get(&prop_name), &prop_value) {
            (Some(prop), Some(JsonValueMemberMatcher::Exact(prop_value_matcher))) => {
                prop_value_matches_exact(prop, prop_value_matcher)
            }
            (Some(prop), Some(JsonValueMemberMatcher::ContainsExact(prop_value_matcher))) => {
                prop_value_contains_exact(prop, prop_value_matcher)
            }
            (Some(prop), Some(JsonValueMemberMatcher::Prefixed(prop_value_matcher))) => {
                prop_value_is_prefixed_by(prop, prop_value_matcher)
            }
            (Some(prop), Some(JsonValueMemberMatcher::Suffixed(prop_value_matcher))) => {
                prop_value_is_suffixed_by(prop, prop_value_matcher)
            }
            (Some(prop), Some(JsonValueMemberMatcher::Contains(prop_value_matcher))) => {
                prop_value_contains(prop, prop_value_matcher)
            }
            (Some(prop), None) => Some(prop),
            (None, _) => None,
        },
        _ => None,
    })
}

fn match_prop(pattern: &str) -> Option<(&str, Option<JsonValueMemberMatcher>, Option<&str>)> {
    lazy_static! {
      static ref re_prop: Regex =
        Regex::new(r#"^\.(?P<prop>([[:word:]])+)(?P<remainder>.+)?$"#).unwrap();
      static ref re_prop_value: Regex = Regex::new(
        concat!(r#"^\{"(?P<prop>([[:word:]])+)"("#,r#"(?P<matchingStrategy>(:|~:|\$:|\^:|\*:)+)"#,r#"("(?P<stringValue>([^"])+)"|(?P<numberValue>([[:digit:]]+)+)|(?P<literalValue>([[:word:]])+)))?\}(?P<remainder>.+)?$"#)
      )
      .unwrap();
    }

    match re_prop
        .captures(pattern)
        .or(re_prop_value.captures(pattern))
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

pub fn greedily_matches(
    maybe_pattern: Option<&str>,
) -> Result<
    (
        Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>>,
        Option<&str>,
    ),
    Option<&str>,
> {
    match maybe_pattern {
        Some(pattern) => match match_prop(pattern) {
            Some((prop_name, prop_value, remainder)) => {
                Ok((prop(String::from(prop_name), prop_value), remainder))
            }
            None => Err(maybe_pattern),
        },
        None => Err(maybe_pattern),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use json::object;

    #[test]
    fn should_match_prop() {
        let res = greedily_matches(Some(".name"));
        assert!(res.is_ok());

        let ref data = object! {
            "name"    => "John Doe",
            "age"     => 30
        };

        match res {
            Ok((matcher, _)) => assert_eq!(matcher(Some(data)), Some(&data["name"])),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn shouldnt_match_identity() {
        let res = greedily_matches(Some("."));
        assert!(res.is_err());
        match res {
            Err(Some(selector)) => assert_eq!(selector, "."),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_return_remainder_when_it_matches_prop() {
        let res = greedily_matches(Some(".father.title"));
        assert!(res.is_ok());

        match res {
            Ok((_, umatched)) => assert_eq!(umatched, Some(".title")),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_return_none_when_json_isnt_present() {
        assert_eq!(prop(String::from(".id"), None)(None), None);
    }

    #[test]
    fn should_return_json_prop_when_json_has_prop() {
        let ref data = object! {
            "name"    => "John Doe",
            "age"     => 30
        };

        assert_eq!(
            prop(String::from("name"), None)(Some(data)),
            Some(&data["name"])
        );
    }

    #[test]
    fn should_match_number_prop() {
        let res = greedily_matches(Some(r#"{"age":30}"#));
        assert!(res.is_ok());

        let ref data = object! {
            "name"    => "John Doe",
            "age"     => 30
        };

        match res {
            Ok((matcher, _)) => assert_eq!(matcher(Some(data)), Some(&data["age"])),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_match_string_property_value_when_using_exact_matching_strategy() {
        let res = greedily_matches(Some(r#"{"country":"IRL"}"#));
        assert!(res.is_ok());

        let ref data = object! {
            "name"      => "John Doe",
            "age"       => 30,
            "country"   => "IRL"
        };

        match res {
            Ok((matcher, _)) => assert_eq!(matcher(Some(data)), Some(&data["country"])),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_match_string_property_value_when_using_contains_exact_matching_strategy() {
        let res = greedily_matches(Some(r#"{"country"~:"GBR"}"#));
        assert!(res.is_ok());

        let ref data = object! {
            "name"      => "John Doe",
            "age"       => 30,
            "country"   => "IRL GBR"
        };

        match res {
            Ok((matcher, _)) => assert_eq!(matcher(Some(data)), Some(&data["country"])),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_match_string_property_value_when_using_prefixed_matching_strategy() {
        let res = greedily_matches(Some(r#"{"country"^:"IRL"}"#));
        assert!(res.is_ok());

        let ref data = object! {
            "name"      => "John Doe",
            "age"       => 30,
            "country"   => "IRL GBR"
        };

        match res {
            Ok((matcher, _)) => assert_eq!(matcher(Some(data)), Some(&data["country"])),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_match_string_property_value_when_using_suffixed_matching_strategy() {
        let res = greedily_matches(Some(r#"{"country"$:"GBR"}"#));
        assert!(res.is_ok());

        let ref data = object! {
            "name"      => "John Doe",
            "age"       => 30,
            "country"   => "IRL GBR"
        };

        match res {
            Ok((matcher, _)) => assert_eq!(matcher(Some(data)), Some(&data["country"])),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_match_boolean_false_prop() {
        let res = greedily_matches(Some(r#"{"is_known":false}"#));
        assert!(res.is_ok());

        let ref data = object! {
            "name"      => "John Doe",
            "age"       => 30,
            "is_known"  => false
        };

        match res {
            Ok((matcher, _)) => assert_eq!(matcher(Some(data)), Some(&data["is_known"])),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_match_boolean_true_prop() {
        let res = greedily_matches(Some(r#"{"is_anonymous":true}"#));
        assert!(res.is_ok());

        let ref data = object! {
            "name"          => "John Doe",
            "age"           => 30,
            "is_anonymous"  => true
        };

        match res {
            Ok((matcher, _)) => assert_eq!(matcher(Some(data)), Some(&data["is_anonymous"])),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_match_null_prop() {
        let res = greedily_matches(Some(r#"{"identity":null}"#));
        assert!(res.is_ok());

        let ref data = object! {
            "name"          => "John Doe",
            "age"           => 30,
            "identity"      => JsonValue::Null
        };

        match res {
            Ok((matcher, _)) => assert_eq!(matcher(Some(data)), Some(&data["identity"])),
            _ => panic!("Invalid result"),
        }
    }
}
