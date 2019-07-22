use json::JsonValue;

pub type FnJsonValueLens = Fn(Option<&JsonValue>) -> Option<&JsonValue>;
pub trait SelectionLens {
    fn select<'a>(&self, input: Option<&'a JsonValue>) -> Option<&'a JsonValue>;
}

pub enum SelectionJsonValueLens {
    Fn(Box<FnJsonValueLens>),
    Lens(Box<SelectionLens>),
}

mod array_member;
mod identity;
mod prop;
mod sequence;
mod value_matchers;

pub fn match_json_slice<'a>(
    matchers: &Vec<SelectionJsonValueLens>,
    json_input: &'a JsonValue,
    match_root_only: bool,
) -> Result<&'a JsonValue, ()> {
    match matchers
        .iter()
        .try_fold(json_input, |json_slice, matcher| match matcher {
            SelectionJsonValueLens::Fn(func) => func(Some(&json_slice)),
            SelectionJsonValueLens::Lens(func) => func.select(Some(&json_slice)),
        }) {
        Some(matching_slice) => Ok(matching_slice),
        None => match (match_root_only, json_input) {
            (false, JsonValue::Object(ref object)) => match object
                .iter()
                .map(|(_, value)| match_json_slice(matchers, value, match_root_only))
                .find(|res| res.is_ok())
            {
                Some(Ok(matching_slice)) => Ok(matching_slice),
                _ => Err(()),
            },
            (false, JsonValue::Array(ref sequence)) => match sequence
                .iter()
                .map(|value| match_json_slice(matchers, value, match_root_only))
                .find(|res| res.is_ok())
            {
                Some(Ok(matching_slice)) => Ok(matching_slice),
                _ => Err(()),
            },
            (_, _) => Err(()),
        },
    }
}

pub fn match_filter(filter: &str) -> Result<(SelectionJsonValueLens, Option<&str>), &str> {
    identity::greedily_matches(Some(filter))
        .or_else(|unmatched_filter| prop::greedily_matches(unmatched_filter))
        .or_else(|unmatched_filter| array_member::greedily_matches(unmatched_filter))
        .or_else(|unmatched_filter| sequence::greedily_matches(unmatched_filter))
        .map_err(|_| filter)
}

pub fn try_to_match_filters(filter: &str) -> Result<Vec<SelectionJsonValueLens>, &str> {
    let mut matchers: Vec<SelectionJsonValueLens> = vec![];
    let mut unmatched_filter: Result<Option<&str>, &str> = Ok(Some(filter));
    while let Ok(Some(filter)) = unmatched_filter {
        match match_filter(filter) {
            Ok((matcher, remainder)) => {
                matchers.push(matcher);
                unmatched_filter = Ok(remainder);
            }
            Err(remaining_filter) => {
                unmatched_filter = Err(remaining_filter);
            }
        };
    }
    match unmatched_filter {
        Ok(None) => Ok(matchers),
        Ok(Some(remaining_filter)) => Err(remaining_filter),
        Err(remaining_filter) => Err(remaining_filter),
    }
}

pub fn match_filters(filter: &str) -> Result<Vec<SelectionJsonValueLens>, String> {
    try_to_match_filters(filter)
        .map_err(|unmatched_filter| format!("Invalid filter: {:?}", unmatched_filter))
}
