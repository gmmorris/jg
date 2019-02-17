use json::*;
mod identity;
mod prop;

pub fn match_filters(filter: &str) -> Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>> {
  match identity::greedily_matches(Some(filter)) {
    Ok((matcher, _)) => matcher,
    Err(unmatched_filter) => match prop::greedily_matches(unmatched_filter) {
      Ok((matcher, _)) => matcher,
      Err(unmatched_filter) => {
        panic!("Invalid filter: {:?}", unmatched_filter);
      }
    },
  }
}
