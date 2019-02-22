pub enum JsonValueMatcher {
  String(JsonValueStringMatcher),
  Number(i64),
  Boolean(bool),
  Null,
}

pub enum JsonValueStringMatcher {
  ExactString(String),
}

fn identify_string_matcher(cap: &regex::Captures) -> Option<Result<JsonValueMatcher, ()>> {
  cap
    .name("stringValue")
    .map(|value| value)
    .map(|value| {
      JsonValueMatcher::String(JsonValueStringMatcher::ExactString(String::from(
        value.as_str(),
      )))
    })
    .map(|string_value| Ok(string_value))
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

pub fn identify_value_matcher(cap: &regex::Captures) -> Result<Option<JsonValueMatcher>, ()> {
  match identify_string_matcher(cap)
    .or(identify_number_matcher(cap))
    .or(identify_literal_matcher(cap))
  {
    Some(Ok(json_value_matcher)) => Ok(Some(json_value_matcher)),
    Some(Err(_)) => Err(()),
    None => Ok(None),
  }
}
