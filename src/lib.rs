#[macro_use]
extern crate lazy_static;
extern crate isatty;
extern crate json_highlight_writer;
extern crate regex;

pub mod input;
mod selection;

pub fn json_grep(config: input::Config) -> Result<(), Option<String>> {
    let lens_patterns = match config.params {
        Some(ref params) => {
            input::parameter_substitution::apply_substitution(&config.matchers, params)
        }
        None => config.matchers.iter().map(|s| s.to_string()).collect(),
    };

    let matched_filters: Result<Vec<_>, String> = lens_patterns
        .iter()
        .map(|pattern| input::in_configured_case(pattern, &config))
        .map(|pattern| selection::match_filters(&pattern))
        .collect();

    let matched_filters = matched_filters?;

    let has_matched = input::scan_input_for_matching_lines(
        &config,
        &|line| {
            invert_result(
                config.invert_match,
                input::match_line(&matched_filters, &config, line),
            )
        },
        &|(index, matched_count, matched_result)| {
            if let Ok(matched_line) = matched_result {
                if !(config.print_only_count || config.is_quiet_mode) {
                    println!(
                        "{}{}",
                        index
                            .map(|index| index.to_string() + ":")
                            .unwrap_or(String::from("")),
                        matched_line
                    );
                };
            };
            (index, matched_count)
        },
    );

    match has_matched {
        Ok(match_count) => {
            if config.print_only_count {
                println!("{}", match_count.expect("failed to count matched input"));
            }
            Ok(())
        }
        Err(err) => Err(err),
    }
}

pub fn invert_result<A>(should_invert: bool, result: Result<A, A>) -> Result<A, A> {
    match result {
        Ok(ok_res) => {
            if should_invert {
                Err(ok_res)
            } else {
                Ok(ok_res)
            }
        }
        Err(err_res) => {
            if should_invert {
                Ok(err_res)
            } else {
                Err(err_res)
            }
        }
    }
}
