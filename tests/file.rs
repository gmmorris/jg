#[cfg(test)]
mod cli {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::io::Write;
    use std::process::Command;

    #[test]
    fn should_read_input_file_when_provided() {
        let mut cmd = Command::main_binary().unwrap();

        let mut tmp_file = tempfile::NamedTempFile::new().unwrap();
        tmp_file
            .write_all(include_str!("./input/events.json").as_bytes())
            .unwrap();

        cmd.arg(".").arg("-f").arg(&tmp_file.path());

        cmd.assert()
            .success()
            .stdout(include_str!("./input/events.output.json"));
    }

    #[test]
    fn should_reject_invalid_json_when_reading_a_file() {
        let mut cmd = Command::main_binary().unwrap();

        let mut tmp_file = tempfile::NamedTempFile::new().unwrap();
        tmp_file
            .write_all(include_str!("./input/invalid_events.json").as_bytes())
            .unwrap();

        cmd.arg(".").arg("-f").arg(&tmp_file.path());

        cmd.assert()
            .success()
            .stdout(include_str!("./input/invalid_events.output.json"));
    }

    #[test]
    fn should_panic_when_the_specified_file_is_missing() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(".").arg("-f").arg("./missing_file.json");

        cmd.assert().failure().stderr(
            predicate::str::is_match(
                r#"The specified input file could not be found: "./missing_file.json""#,
            )
            .unwrap(),
        );
    }
}
