pub fn enumeration(
  enumerate_all: bool,
  enumerate_oks: bool,
) -> Box<FnMut(Result<String, String>) -> (Option<usize>, Option<usize>, Result<String, String>)> {
  let mut all: Option<usize> = None;
  let mut oks: Option<usize> = None;
  Box::new(move |result: Result<String, String>| {
    (
      {
        if enumerate_all {
          all = all.or(Some(0)).and_then(|count| count.checked_add(1));
        }
        all
      },
      {
        if enumerate_oks && result.is_ok() {
          oks = oks.or(Some(0)).and_then(|count| count.checked_add(1));
        }
        oks
      },
      result,
    )
  })
}
