#[cfg(test)]
mod cli {
    use std::process::Command;
    use assert_cmd::prelude::*;
    use tempfile;
    use std::io::Write;

    #[test]
    fn should_read_input_file_when_provided() {
        let mut cmd = Command::main_binary().unwrap();

        let mut tmp_file = tempfile::NamedTempFile::new().unwrap();
        tmp_file.write_all(include_str!("./input/events.json").as_bytes()).unwrap();

        cmd
          .arg(".")
          .arg("-i")
          .arg(&tmp_file.path());

        cmd
            .assert()
            .success()
            .stdout(include_str!("./input/events.output.json"));
    }
}