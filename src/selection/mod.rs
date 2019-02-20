use json::JsonValue;
mod array_index;
mod identity;
mod prop;

pub fn match_filter(
  filter: &str,
) -> Result<
  (
    Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>>,
    Option<&str>,
  ),
  &str,
> {
  match identity::greedily_matches(Some(filter)) {
    Ok((matcher, remainder)) => Ok((matcher, remainder)),
    Err(unmatched_filter) => match prop::greedily_matches(unmatched_filter) {
      Ok((matcher, remainder)) => Ok((matcher, remainder)),
      Err(unmatched_filter) => match array_index::greedily_matches(unmatched_filter) {
        Ok((matcher, remainder)) => Ok((matcher, remainder)),
        Err(_) => Err(filter),
      },
    },
  }
}

pub fn match_json_slice(
  matchers: &Vec<Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>>>,
  json_input: &JsonValue,
) -> Result<(), ()> {
  match json_input {
    JsonValue::Object(_) | JsonValue::Array(_) => match matchers
      .iter()
      .try_fold(json_input, |json_slice, matcher| matcher(Some(&json_slice)))
    {
      Some(_) => Ok(()),
      None => match json_input {
        JsonValue::Object(ref object) => match object
          .iter()
          .find(|(_, value)| match_json_slice(matchers, *value).is_ok())
        {
          Some(_) => Ok(()),
          None => Err(()),
        },
        _ => Err(()),
      },
    },
    _ => Err(()),
  }
}

pub fn match_filters(filter: &str) -> Vec<Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>>> {
  let mut matchers: Vec<Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>>> = vec![];
  let mut unmatched_filter: Result<Option<&str>, &str> = Ok(Some(filter));
  while let Ok(Some(filter)) = unmatched_filter {
    match match_filter(filter) {
      Ok((matcher, remainder)) => {
        matchers.push(matcher);
        unmatched_filter = Ok(remainder);
      }
      Err(unmatched_filter) => {
        panic!("Invalid filter: {:?}", unmatched_filter);
      }
    };
  }
  matchers
}
