use json::JsonValue;
use regex::Regex;

pub fn array_index(index: usize) -> Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>> {
  Box::new(move |input: Option<&JsonValue>| match input {
    Some(json) => match json {
      JsonValue::Array(ref array) => array.get(index),
      _ => None,
    },
    None => None,
  })
}

fn match_array_index(pattern: &str) -> Option<(usize, Option<&str>)> {
  lazy_static! {
    static ref re_prop: Regex =
      Regex::new(r#"^\[(?P<index>([[:digit:]])+)\](?P<remainder>.+)?$"#).unwrap();
  }

  re_prop.captures(pattern).and_then(|cap| {
    cap
      .name("index")
      .map(|index| index.as_str())
      .map(|index| usize::from_str_radix(index, 32))
      .filter(|index| index.is_ok())
      .map(|index| {
        (
          index.unwrap(),
          cap.name("remainder").map(|remainder| remainder.as_str()),
        )
      })
  })
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
      Some((index, remainder)) => Ok((array_index(index), remainder)),
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
}
