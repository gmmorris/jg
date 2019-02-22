use json::JsonValue;
use regex::Regex;

use super::value_matchers::*;

enum ArrayMember {
  Index(usize),
  Value(JsonValueMemberMatcher),
}

pub fn array_index(index: usize) -> Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>> {
  Box::new(move |input: Option<&JsonValue>| match input {
    Some(json) => match json {
      JsonValue::Array(ref array) => array.get(index),
      _ => None,
    },
    None => None,
  })
}

fn member_in_array<'a>(
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
      (JsonValue::Number(num_prop), JsonValueMatcher::Number(num_value)) => num_prop.eq(num_value),
      (JsonValue::Null, JsonValueMatcher::Null) => true,
      _ => false,
    })
}

pub fn array_member(
  member_matcher: JsonValueMemberMatcher,
) -> Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>> {
  Box::new(move |input: Option<&JsonValue>| match input {
    Some(json) => match json {
      JsonValue::Array(ref array) => match &member_matcher {
        JsonValueMemberMatcher::Exact(json_value_matcher) => {
          if array.len() == 1 {
            member_in_array(array, json_value_matcher)
          } else {
            None
          }
        }
        JsonValueMemberMatcher::ContainsExact(json_value_matcher) => {
          member_in_array(array, json_value_matcher)
        }
      },
      _ => None,
    },
    None => None,
  })
}

fn match_array_index(pattern: &str) -> Option<(ArrayMember, Option<&str>)> {
  lazy_static! {
    static ref re_index: Regex =
      Regex::new(r#"^\[(?P<index>([[:digit:]])+)\](?P<remainder>.+)?$"#).unwrap();
    static ref re_member: Regex = Regex::new(
      concat!(r#"^\["#,r#"(?P<matchingStrategy>(~=|=)+)"#,r#"("(?P<stringValue>([^"])+)"|(?P<numberValue>([[:digit:]]+)+)|(?P<literalValue>([[:word:]])+))\](?P<remainder>.+)?$"#)
    )
    .unwrap();
  }

  match re_index.captures(pattern) {
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
    None => match re_member.captures(pattern) {
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
    Some(pattern) => match match_array_index(pattern) {
      Some((ArrayMember::Index(index), remainder)) => Ok((array_index(index), remainder)),
      Some((ArrayMember::Value(value_matcher), remainder)) => {
        Ok((array_member(value_matcher), remainder))
      }
      None => Err(maybe_pattern),
    },
    None => Err(maybe_pattern),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use json::array;
  use json::object;

  #[test]
  fn should_match_array_index() {
    let res = greedily_matches(Some("[0]"));
    assert!(res.is_ok());

    let ref data = array![object! {
        "name"    => "John Doe",
        "age"     => 30
    }];

    match res {
      Ok((matcher, _)) => assert_eq!(matcher(Some(data)), Some(&data[0])),
      _ => panic!("Invalid result"),
    }
  }

  #[test]
  fn should_return_remainder_when_it_matches_index() {
    let res = greedily_matches(Some("[0].title"));
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

    assert_eq!(array_index(0)(Some(data)), Some(&data[0]));
  }

  #[test]
  fn should_return_none_when_json_isnt_present() {
    assert_eq!(array_index(10)(None), None);
  }

  #[test]
  fn should_return_node_when_string_value_is_present_in_array() {
    let ref data = array!["John Doe", "Jane Doe", "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."];

    assert_eq!(
      array_member(JsonValueMemberMatcher::ContainsExact(
        JsonValueMatcher::String(String::from("Jane Doe"))
      ))(Some(data)),
      Some(&data[1])
    );

    assert_eq!(
      array_member(JsonValueMemberMatcher::ContainsExact(JsonValueMatcher::String(String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."))))(Some(data)),
      Some(&data[2])
    );
  }

  #[test]
  fn should_return_node_when_boolean_value_is_present_in_array() {
    let ref data = array![true, false];

    assert_eq!(
      array_member(JsonValueMemberMatcher::ContainsExact(
        JsonValueMatcher::Boolean(true)
      ))(Some(data)),
      Some(&data[0])
    );

    assert_eq!(
      array_member(JsonValueMemberMatcher::ContainsExact(
        JsonValueMatcher::Boolean(false)
      ))(Some(data)),
      Some(&data[1])
    );
  }

  #[test]
  fn should_return_node_when_null_value_is_present_in_array() {
    let ref data = array![true, JsonValue::Null];

    assert_eq!(
      array_member(JsonValueMemberMatcher::ContainsExact(
        JsonValueMatcher::Null
      ))(Some(data)),
      Some(&data[1])
    );
  }

  #[test]
  fn should_return_node_when_numeric_value_is_present_in_array() {
    let ref data = array![0, -10, 10, 123456789];

    assert_eq!(
      array_member(JsonValueMemberMatcher::ContainsExact(
        JsonValueMatcher::Number(0)
      ))(Some(data)),
      Some(&data[0])
    );

    assert_eq!(
      array_member(JsonValueMemberMatcher::ContainsExact(
        JsonValueMatcher::Number(-10)
      ))(Some(data)),
      Some(&data[1])
    );

    assert_eq!(
      array_member(JsonValueMemberMatcher::ContainsExact(
        JsonValueMatcher::Number(10)
      ))(Some(data)),
      Some(&data[2])
    );

    assert_eq!(
      array_member(JsonValueMemberMatcher::ContainsExact(
        JsonValueMatcher::Number(123456789)
      ))(Some(data)),
      Some(&data[3])
    );
  }
}
