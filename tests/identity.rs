#[cfg(test)]
mod cli {
    use std::process::Command;

    use assert_cmd::prelude::*;

    #[test]
    fn should_match_single_json_when_selector_is_identity() {
        let mut cmd = Command::main_binary().unwrap();

        cmd
            .arg(".");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{}\n");

        assert_cmd
            .assert()
            .success()
            .stdout("{}\n");
    }

    #[test]
    fn should_match_all_json_when_selector_is_identity() {
        let mut cmd = Command::main_binary().unwrap();

        cmd
            .arg(".");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
"{}
{ \"id\": \"404c18ce-04ac-457c-99f5-d548b27aa583\" }\n"
        );

        assert_cmd
            .assert()
            .success()
            .stdout(
"{}
{ \"id\": \"404c18ce-04ac-457c-99f5-d548b27aa583\" }\n"
            );

    }

    #[test]
    fn should_not_match_all_when_selector_is_identity() {
        let mut cmd = Command::main_binary().unwrap();

        cmd
            .arg(".prop");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{}\n");

        assert_cmd
            .assert()
            .success()
            .stdout("");
    }
}