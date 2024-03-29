use json::JsonValue;

pub trait SelectionLens {
    fn select<'a>(&self, input: Option<&'a JsonValue>) -> Option<&'a JsonValue>;
}

pub trait SelectionLensParser {
    fn try_parse<'a>(
        &self,
        lens_pattern: Option<&'a str>,
    ) -> Result<(Box<dyn SelectionLens>, Option<&'a str>), Option<&'a str>>;
}

mod array_member;
mod identity;
mod prop;
mod sequence;
mod value_matchers;

pub fn match_json_slice<'a>(
    matchers: &Vec<Box<dyn SelectionLens>>,
    json_input: &'a JsonValue,
    match_root_only: bool,
) -> Result<&'a JsonValue, ()> {
    match matchers.iter().try_fold(json_input, |json_slice, matcher| {
        matcher.select(Some(json_slice))
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

pub fn match_filter(filter: &str) -> Result<(Box<dyn SelectionLens>, Option<&str>), &str> {
    lazy_static! {
        static ref IDENTITY_PARSER: identity::IdentityParser = identity::IdentityParser {};
        static ref PROP_PARSER: prop::PropParser = prop::PropParser {};
        static ref ARRAY_MEMBER_PARSER: array_member::ArrayMemberParser =
            array_member::ArrayMemberParser {};
        static ref SEQUENCE_PARSER: sequence::SequenceParser = sequence::SequenceParser {};
    }

    IDENTITY_PARSER
        .try_parse(Some(filter))
        .or_else(|unmatched_filter| PROP_PARSER.try_parse(unmatched_filter))
        .or_else(|unmatched_filter| ARRAY_MEMBER_PARSER.try_parse(unmatched_filter))
        .or_else(|unmatched_filter| SEQUENCE_PARSER.try_parse(unmatched_filter))
        .map_err(|_| filter)
}

pub fn try_to_match_filters(filter: &str) -> Result<Vec<Box<dyn SelectionLens>>, &str> {
    let mut matchers: Vec<Box<dyn SelectionLens>> = vec![];
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

pub fn match_filters(filter: &str) -> Result<Vec<Box<dyn SelectionLens>>, String> {
    try_to_match_filters(filter)
        .map_err(|unmatched_filter| format!("Invalid filter: {:?}", unmatched_filter))
}
