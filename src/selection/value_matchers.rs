#[derive(Debug)]
pub enum JsonValueMatcher {
  ExactString(String),
  Number(i64),
  Boolean(bool),
  Null,
}
