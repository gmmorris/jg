use json::JsonValue;
use regex::Regex;

use super::value_matchers::JsonValueMatcher;

pub fn prop(
  prop_name: String,
  prop_value: Option<JsonValueMatcher>,
) -> Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>> {
  Box::new(move |input: Option<&JsonValue>| match input {
    Some(json) => match json {
      JsonValue::Object(ref object) => match object.get(&prop_name) {
        Some(prop) => match (prop, &prop_value) {
          (JsonValue::String(string_prop), Some(JsonValueMatcher::ExactString(prop_value))) => {
            Some(prop).filter(|_| string_prop.eq(prop_value))
          }
          (JsonValue::Short(string_prop), Some(JsonValueMatcher::ExactString(prop_value))) => {
            Some(prop).filter(|_| string_prop.eq(prop_value))
          }
          (JsonValue::Number(number_prop), Some(JsonValueMatcher::Number(prop_value))) => {
            Some(prop).filter(|_| number_prop.eq(prop_value))
          }
          (JsonValue::Boolean(bool_prop), Some(JsonValueMatcher::Boolean(prop_value))) => {
            Some(prop).filter(|_| bool_prop.eq(prop_value))
          }
          (JsonValue::Null, Some(JsonValueMatcher::Null)) => Some(prop),
          (_, Some(_)) => None,
          (_, None) => Some(prop),
        },
        None => None,
      },
      _ => None,
    },
    None => None,
  })
}

fn identify_value_matcher(cap: &regex::Captures) -> Result<Option<JsonValueMatcher>, ()> {
  let string_matcher = cap
    .name("stringValue")
    .map(|value| value)
    .map(|value| JsonValueMatcher::ExactString(String::from(value.as_str())))
    .map(|string_value| Ok(string_value));

  let number_matcher =
    cap
      .name("numberValue")
      .map(|value| match String::from(value.as_str()).parse::<i64>() {
        Ok(number_value) => Ok(JsonValueMatcher::Number(number_value)),
        Err(_) => Err(()),
      });

  let literal_matcher = cap.name("literalValue").map(|value| match value.as_str() {
    "true" => Ok(JsonValueMatcher::Boolean(true)),
    "false" => Ok(JsonValueMatcher::Boolean(false)),
    "null" => Ok(JsonValueMatcher::Null),
    _ => Err(()),
  });

  match string_matcher.or(number_matcher).or(literal_matcher) {
    Some(Ok(json_value_matcher)) => Ok(Some(json_value_matcher)),
    Some(Err(_)) => Err(()),
    None => Ok(None),
  }
}

fn match_prop(pattern: &str) -> Option<(&str, Option<JsonValueMatcher>, Option<&str>)> {
  lazy_static! {
    static ref re_prop: Regex =
      Regex::new(r#"^\.(?P<prop>([[:word:]])+)(?P<remainder>.+)?$"#).unwrap();
    static ref re_prop_value: Regex = Regex::new(
      r#"^\{"(?P<prop>([[:word:]])+)"(:("(?P<stringValue>([^"])+)"|(?P<numberValue>([[:digit:]]+)+)|(?P<literalValue>([[:word:]])+)))?\}(?P<remainder>.+)?$"#
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
