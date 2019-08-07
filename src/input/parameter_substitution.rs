pub fn apply_substitution(sources: &Vec<&str>, params: &Vec<&str>) -> Vec<String> {
    let mut params =  params.iter().peekable();
    sources
        .iter()
        .map(move |&src| {
            let mut src = String::from(src);
            if params.peek().is_some() {
                while let Some(_) = src.find("{}") {
                    if let Some(param) = params.next() {
                        src = src.replacen("{}", param, 1);
                    } else {
                        break;
                    }
                };
            }
            src
        })  
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substitution_should_return_string_as_is_if_no_parameters_are_present() {
        assert_eq!(
            vec!["Jeff"],
            apply_substitution(&vec!["Jeff"],&vec![])
        );
    }

    #[test]
    fn substitution_should_return_string_as_is_if_no_substitution_flag_is_present() {
        assert_eq!(
            vec!["Jeff"],
            apply_substitution(&vec!["Jeff"],&vec!["Goldbloom"])
        );
    }

    #[test]
    fn substitution_should_replace_a_single_substitution_flag() {
        assert_eq!(
            vec!["Jeff Goldbloom"],
            apply_substitution(&vec!["Jeff {}"],&vec!["Goldbloom"])
        );
    }

    #[test]
    fn substitution_should_replace_multiple_substitution_flag() {
        assert_eq!(
            vec!["Jeff Goldbloom"],
            apply_substitution(&vec!["{} {}"],&vec!["Jeff", "Goldbloom"])
        );
    }

    #[test]
    fn substitution_leave_substitution_flag_untouched_if_tere_are_no_more_parameters() {
        assert_eq!(
            vec!["Jeff {}"],
            apply_substitution(&vec!["{} {}"],&vec!["Jeff"])
        );
    }

    #[test]
    fn substitution_should_replace_across_multiple_sources() {
        assert_eq!(
            vec!["Jeff","Goldbloom"],
            apply_substitution(&vec!["{}","{}"],&vec!["Jeff", "Goldbloom"])
        );
    }
}
