use json::JsonValue;
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
      Err(_) => Err(filter),
    },
  }
}

pub fn match_filters(filter: &str) -> Vec<Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>>> {
  let mut matchers: Vec<Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>>> = vec![];
  let mut unmatched_filter: Result<Option<&str>, &str> = Ok(Some(filter));
  while let Ok(Some(filter)) = unmatched_filter {
    match match_filter(filter) {
      Ok((matcher, remainder)) => {
        matchers.push(matcher);
        match remainder {
          None => {
            unmatched_filter = Ok(None);
          }
          Some("") => {
            unmatched_filter = Ok(None);
          }
          Some(_) => {
            unmatched_filter = Ok(remainder);
          }
        }
      }
      Err(unmatched_filter) => {
        panic!("Invalid filter: {:?}", unmatched_filter);
      }
    };
  }
  matchers
}
