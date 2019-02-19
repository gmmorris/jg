use json::JsonValue;
use regex::Regex;

pub fn prop(
  prop_name: String,
  prop_value: Option<JsonValue>,
) -> Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>> {
  Box::new(move |input: Option<&JsonValue>| match input {
    Some(json) => match json {
      JsonValue::Object(ref object) => match (object.get(&prop_name), &prop_value) {
        (Some(prop), Some(prop_value)) => {
          if prop.eq(prop_value) {
            Some(prop)
          } else {
            None
          }
        }
        (Some(prop), None) => Some(prop),
        (_, _) => None,
      },
      _ => None,
    },
    None => None,
  })
}

fn match_prop(pattern: &str) -> Option<(&str, Option<JsonValue>, Option<&str>)> {
  lazy_static! {
    static ref re_prop: Regex =
      Regex::new(r#"^\.(?P<prop>([[:word:]])+)(?P<remainder>.+)?$"#).unwrap();
    static ref re_index_prop: Regex = Regex::new(
      r#"^\{"(?P<prop>([[:word:]])+)"(:"(?P<stringValue>([^"])+)")?\}(?P<remainder>.+)?$"#
    )
    .unwrap();
  }

  match re_prop
    .captures(pattern)
    .or(re_index_prop.captures(pattern))
  {
    Some(cap) => cap.name("prop").map(|prop| {
      (
        prop.as_str(),
        cap
          .name("stringValue")
          .map(|value| JsonValue::String(String::from(value.as_str()))),
        cap.name("remainder").map(|remainder| remainder.as_str()),
      )
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
}
