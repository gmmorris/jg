use json::*;
mod identity;

pub fn match_filters(filter: &str) -> fn(Option<&JsonValue>) -> Option<&JsonValue> {
  let selection_matches = identity::greedily_matches(Some(filter));
  match selection_matches {
    Ok(_) => identity::identity,
    Err(unmatched_filter) => {
      panic!("Invalid filter: {:?}", unmatched_filter);
    }
  }
}
