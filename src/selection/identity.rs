use json::JsonValue;

pub fn identity(input: Option<&JsonValue>) -> Option<&JsonValue> {
  input
}

pub fn greedily_matches(maybe_pattern: Option<&str>) -> Result<Option<&str>, Option<&str>> {
  match maybe_pattern {
    Some(pattern) => match pattern {
      "." => Ok(None),
      _ => Err(maybe_pattern),
    },
    None => Err(maybe_pattern),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use json::object;

  #[test]
  fn should_match_dot() {
    assert_eq!(greedily_matches(Some(".")), Ok(None));
  }

  #[test]
  fn shouldnt_match_anything_else() {
    assert_eq!(greedily_matches(Some(".prop")), Err(Some(".prop")));
  }

  #[test]
  fn should_return_none_when_json_isnt_present() {
    assert_eq!(identity(None), None);
  }

  #[test]
  fn should_return_some_json_when_json_is_present() {
    let ref data = object! {
        "name"    => "John Doe",
        "age"     => 30
    };

    assert_eq!(
      match identity(Some(data)) {
        Some(d) => d,
        None => &json::Null,
      },
      data
    );
  }
}
