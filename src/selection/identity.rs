use json::*;

pub fn identity(input: Option<&JsonValue>) -> Option<&JsonValue> {
  input
}

pub fn matches(pattern: &str) -> bool {
  match pattern {
    "." => true,
    _ => false,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_match_dot() {
    assert_eq!(matches("."), true);
  }

  #[test]
  fn shouldnt_match_anything_else() {
    assert_eq!(matches(".prop"), false);
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
