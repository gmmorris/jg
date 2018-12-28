#[cfg(test)]
mod cli {
    // use std::io::Write;
    use std::process::Command;

    use assert_cmd::prelude::*;
    // use tempfile;

    #[test]
    fn should_print_out_provided_selector() {
        let mut cmd = Command::main_binary().unwrap();

        cmd
            .arg("-v")
            .arg(".");

        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{}\n");

        assert_cmd
            .assert()
            .success()
            .stdout(
"filter: .
-----
{}\n"
              );
    }
}