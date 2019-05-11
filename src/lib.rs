#[macro_use]
extern crate lazy_static;
extern crate isatty;
extern crate json_highlight_writer;
extern crate regex;

pub mod input;
mod selection;

pub fn json_grep(config : input::Config) {
  let matched_filters = config.matchers
      .iter()
      .map(|pattern| input::in_configured_case(pattern, &config))
      .map(|pattern| selection::match_filters(&pattern))
      .collect::<Vec<_>>();

  let has_matched = input::match_input(
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
          std::process::exit(0);
      }
      Err(_) => {
          std::process::exit(1);
      }
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
