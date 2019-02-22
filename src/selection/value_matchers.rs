pub enum JsonValueMatcher {
  String(String),
  Number(i64),
  Boolean(bool),
  Null,
}

pub enum JsonValueMemberMatcher {
  Exact(JsonValueMatcher),
  ContainsExact(JsonValueMatcher),
  Prefixed(JsonValueMatcher),
  Suffixed(JsonValueMatcher),
  Contains(JsonValueMatcher),
}

fn identify_member_matcher(
  cap: &regex::Captures,
  member: JsonValueMatcher,
) -> Result<JsonValueMemberMatcher, ()> {
  match cap.name("matchingStrategy").map(|value| value.as_str()) {
    Some("~=") | Some("~:") => Ok(JsonValueMemberMatcher::ContainsExact(member)),
    Some("^=") | Some("^:") => Ok(JsonValueMemberMatcher::Prefixed(member)),
    Some("$=") | Some("$:") => Ok(JsonValueMemberMatcher::Suffixed(member)),
    Some("=") | Some(":") => Ok(JsonValueMemberMatcher::Exact(member)),
    Some("*=") | Some("*:") => Ok(JsonValueMemberMatcher::Contains(member)),
    _ => Err(()),
  }
}

fn identify_string_matcher(cap: &regex::Captures) -> Option<Result<JsonValueMatcher, ()>> {
  cap
    .name("stringValue")
    .map(|value| Ok(JsonValueMatcher::String(String::from(value.as_str()))))
}

fn identify_number_matcher(cap: &regex::Captures) -> Option<Result<JsonValueMatcher, ()>> {
  cap
    .name("numberValue")
    .map(|value| match String::from(value.as_str()).parse::<i64>() {
      Ok(number_value) => Ok(JsonValueMatcher::Number(number_value)),
      Err(_) => Err(()),
    })
}

fn identify_literal_matcher(cap: &regex::Captures) -> Option<Result<JsonValueMatcher, ()>> {
  cap.name("literalValue").map(|value| match value.as_str() {
    "true" => Ok(JsonValueMatcher::Boolean(true)),
    "false" => Ok(JsonValueMatcher::Boolean(false)),
    "null" => Ok(JsonValueMatcher::Null),
    _ => Err(()),
  })
}

pub fn identify_value_matcher(cap: &regex::Captures) -> Result<Option<JsonValueMemberMatcher>, ()> {
  match identify_string_matcher(cap)
    .or(identify_number_matcher(cap))
    .or(identify_literal_matcher(cap))
  {
    Some(Ok(json_value_matcher)) => match identify_member_matcher(cap, json_value_matcher) {
      Ok(match_strategy) => Ok(Some(match_strategy)),
      _ => Err(()),
    },
    Some(Err(_)) => Err(()),
    None => Ok(None),
  }
}
