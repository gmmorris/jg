pub struct Enumeration {
    enumerate_all: bool,
    enumerate_oks: bool,
    all: Option<usize>,
    oks: Option<usize>,
}

impl Enumeration {
    pub fn new(enumerate_all: bool, enumerate_oks: bool) -> Enumeration {
        Enumeration {
            enumerate_all,
            enumerate_oks,
            all: None,
            oks: None,
        }
    }

    pub fn enumerate(
        &mut self,
        result: Result<String, String>,
    ) -> (Option<usize>, Option<usize>, Result<String, String>) {
        (
            {
                if self.enumerate_all {
                    self.all = self.all.or(Some(0)).and_then(|count| count.checked_add(1));
                }
                self.all
            },
            {
                if self.enumerate_oks && result.is_ok() {
                    self.oks = self.oks.or(Some(0)).and_then(|count| count.checked_add(1));
                }
                self.oks
            },
            result,
        )
    }
}
