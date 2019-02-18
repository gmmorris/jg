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
  matchers.push(match match_filter(filter) {
    Ok((matcher, _)) => matcher,
    Err(unmatched_filter) => {
      panic!("Invalid filter: {:?}", unmatched_filter);
    }
  });
  matchers
}
