use json::JsonValue;

use super::{SelectionLens, SelectionLensParser};

struct Identity;
impl SelectionLens for Identity {
    fn select<'a>(&self, input: Option<&'a JsonValue>) -> Option<&'a JsonValue> {
        input
    }
}

pub struct IdentityParser;
impl SelectionLensParser for IdentityParser {
    fn try_parse<'a>(
        &self,
        lens_pattern: Option<&'a str>,
    ) -> Result<(Box<dyn SelectionLens>, Option<&'a str>), Option<&'a str>> {
        match lens_pattern {
            Some(pattern) => match pattern {
                "." => Ok((Box::new(Identity {}), None)),
                _ => Err(lens_pattern),
            },
            None => Err(lens_pattern),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use json::object;

    #[test]
    fn should_match_dot() {
        let identity_parser = IdentityParser {};
        let res = identity_parser.try_parse(Some("."));
        assert!(res.is_ok());

        let ref data = object! {
            "name"    => "John Doe",
            "age"     => 30
        };

        match res {
            Ok((lens, unmatched)) => {
                assert_eq!(lens.select(Some(data)), Some(data));
                assert_eq!(unmatched, None);
            }
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn shouldnt_match_anything_else() {
        let identity_parser = IdentityParser {};
        let res = identity_parser.try_parse(Some(".prop"));
        assert!(res.is_err());
        match res {
            Err(Some(selector)) => assert_eq!(selector, ".prop"),
            _ => panic!("Invalid result"),
        }
    }

    #[test]
    fn should_return_none_when_json_isnt_present() {
        let identity = Identity {};
        assert_eq!(identity.select(None), None);
    }

    #[test]
    fn should_return_some_json_when_json_is_present() {
        let ref data = object! {
            "name"    => "John Doe",
            "age"     => 30
        };

        let identity = Identity {};
        assert_eq!(identity.select(Some(data)).unwrap(), data);
    }
}
