#[derive(Debug)]
pub enum JsonValueMatcher {
  ExactString(String),
  Number(i64),
  Boolean(bool),
  Null,
}

pub fn identify_value_matcher(cap: &regex::Captures) -> Result<Option<JsonValueMatcher>, ()> {
  let string_matcher = cap
    .name("stringValue")
    .map(|value| value)
    .map(|value| JsonValueMatcher::ExactString(String::from(value.as_str())))
    .map(|string_value| Ok(string_value));

  let number_matcher =
    cap
      .name("numberValue")
      .map(|value| match String::from(value.as_str()).parse::<i64>() {
        Ok(number_value) => Ok(JsonValueMatcher::Number(number_value)),
        Err(_) => Err(()),
      });

  let literal_matcher = cap.name("literalValue").map(|value| match value.as_str() {
    "true" => Ok(JsonValueMatcher::Boolean(true)),
    "false" => Ok(JsonValueMatcher::Boolean(false)),
    "null" => Ok(JsonValueMatcher::Null),
    _ => Err(()),
  });

  match string_matcher.or(number_matcher).or(literal_matcher) {
    Some(Ok(json_value_matcher)) => Ok(Some(json_value_matcher)),
    Some(Err(_)) => Err(()),
    None => Ok(None),
  }
}
