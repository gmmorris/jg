use json::JsonValue;

pub fn identity() -> Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>> {
  Box::new(|input: Option<&JsonValue>| input)
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
    Some(pattern) => match pattern {
      "." => Ok((identity(), None)),
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
    let res = greedily_matches(Some("."));
    assert!(res.is_ok());

    let ref data = object! {
        "name"    => "John Doe",
        "age"     => 30
    };

    match res {
      Ok((matcher, unmatched)) => {
        assert_eq!(matcher(Some(data)), Some(data));
        assert_eq!(unmatched, None);
      }
      _ => panic!("Invalid result"),
    }
  }

  #[test]
  fn shouldnt_match_anything_else() {
    let res = greedily_matches(Some(".prop"));
    assert!(res.is_err());
    match res {
      Err(Some(selector)) => assert_eq!(selector, ".prop"),
      _ => panic!("Invalid result"),
    }
  }

  #[test]
  fn should_return_none_when_json_isnt_present() {
    assert_eq!(identity()(None), None);
  }

  #[test]
  fn should_return_some_json_when_json_is_present() {
    let ref data = object! {
        "name"    => "John Doe",
        "age"     => 30
    };

    assert_eq!(identity()(Some(data)).unwrap(), data);
  }
}
