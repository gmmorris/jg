use json::JsonValue;
mod array_member;
mod identity;
mod prop;
mod sequence;
mod value_matchers;

pub fn match_json_slice<'a>(
    matchers: &Vec<Box<Fn(Option<&'a JsonValue>) -> Option<&'a JsonValue>>>,
    json_input: &'a JsonValue,
    match_root_only: bool,
) -> Result<&'a JsonValue, ()> {
    match matchers
        .iter()
        .try_fold(json_input, |json_slice, matcher| matcher(Some(&json_slice)))
    {
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
            Err(unmatched_filter) => match array_member::greedily_matches(unmatched_filter) {
                Ok((matcher, remainder)) => Ok((matcher, remainder)),
                Err(unmatched_filter) => match sequence::greedily_matches(unmatched_filter) {
                    Ok((matcher, remainder)) => Ok((matcher, remainder)),
                    Err(_) => Err(filter),
                },
            },
        },
    }
}

pub fn try_to_match_filters(
    filter: &str,
) -> Result<Vec<Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>>>, &str> {
    let mut matchers: Vec<Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>>> = vec![];
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

pub fn match_filters(filter: &str) -> Vec<Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>>> {
    match try_to_match_filters(filter) {
        Ok(matchers) => matchers,
        Err(unmatched_filter) => {
            panic!("Invalid filter: {:?}", unmatched_filter);
        }
    }
}
